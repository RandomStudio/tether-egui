use std::fs;

use crate::{
    insights::Insights,
    project::TetherSettingsInProject,
    tether_utils::{init_new_tether_agent, EditableTetherSettings},
};
use egui::{Color32, RichText, Ui};
use log::*;
use tether_agent::TetherAgentOptionsBuilder;

use crate::Model;

use self::project_builder::{available_widgets, widgets_in_use};

pub mod project_builder;

pub fn standard_spacer(ui: &mut egui::Ui) {
    ui.add_space(16.);
}

fn common_remove_button(ui: &mut Ui) -> bool {
    ui.button("❌ Remove").clicked()
}

fn attempt_new_tether_connection(model: &mut Model) {
    let tether_options = TetherAgentOptionsBuilder::from(&model.editable_tether_settings);

    let tether_agent = init_new_tether_agent(&tether_options);

    match tether_agent.connect(&tether_options) {
        Ok(_) => {
            info!("Connected Tether Agent OK");
            model.editable_tether_settings.was_changed = true;
            model.insights = Insights::new(&tether_agent, &model.monitor_topic);
        }
        Err(e) => {
            model.editable_tether_settings.is_editing = false;
            model.editable_tether_settings.was_changed = false;

            error!("Failed to connect Tether, {}", e);
        }
    }

    model.tether_agent = tether_agent;
}

// fn number_widget_range(ui: &mut Ui, model: &mut Model, default_max: f32) {
//     let openness = ui
//         .collapsing("Range", |ui| {
//             ui.label("Range");
//             ui.vertical(|ui| {
//                 ui.add(
//                     egui::Slider::new(&mut model.next_range.0, i16::MIN as f32..=i16::MAX as f32)
//                         .text("min"),
//                 );
//                 ui.add(
//                     egui::Slider::new(&mut model.next_range.1, i16::MIN as f32..=i16::MAX as f32)
//                         .text("max"),
//                 );
//                 if ui.small_button("Reset").clicked() {
//                     model.next_range = (0., default_max);
//                 }
//             });
//         })
//         .openness;

//     if openness > 0. && openness < 1.0 {
//         model.next_range = (0., default_max);
//     }
//     ui.end_row();
// }

pub fn general_agent_area(ui: &mut Ui, model: &mut Model) {
    ui.heading("Project");

    standard_spacer(ui);
    ui.separator();
    ui.heading("Load/Save");
    if let Some(json_path) = &model.json_file {
        ui.small(json_path);
    } else {
        ui.small("(No JSON file loaded)");
    }
    ui.horizontal(|ui| {
        if ui.button("Save").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("text", &["json"])
                .save_file()
            {
                if model.editable_tether_settings.was_changed {
                    info!("Tether Settings were edited; saving these to project");
                    model.project.tether_settings = Some(TetherSettingsInProject::from(model.editable_tether_settings.clone()));
                };
                let path_string = path.display().to_string();
                let text = serde_json::to_string_pretty(&model.project)
                    .expect("failed to serialise widget data");
                match fs::write(path_string, text) {
                    Ok(()) => {
                        info!("Saved OK");
                    }
                    Err(e) => {
                        error!("Error saving to disk: {:?}", e);
                    }
                }
            }
        }
        if ui.button("Load").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("text", &["json"])
                .pick_file()
            {
                let path_string = path.display().to_string();
                match model.project.load(&path_string) {
                    Ok(()) => {
                        info!("Loaded project file OK");
                        model.json_file = Some(path_string);
                        if let Some(tether_settings_in_project) = &model.project.tether_settings {
                            info!("Project file had custom Tether settings; attempt to apply and connect...");

                            model.editable_tether_settings = EditableTetherSettings::from(&tether_settings_in_project.clone());
                            attempt_new_tether_connection(model);

                        }
                    }
                    Err(_) => {
                        error!("Failed to load project file");
                    }
                }
            }
        }
        if ui.button("Clear").clicked() {
            model.project.widgets.clear();
            model.json_file = None;
        }
    });

    standard_spacer(ui);
    ui.separator();
    ui.heading("Agent");

    if model.editable_tether_settings.is_editing {
        ui.horizontal(|ui| {
            ui.label("IP Address");
            ui.text_edit_singleline(&mut model.editable_tether_settings.host);
        });
        ui.horizontal(|ui| {
            ui.label("Username");
            ui.text_edit_singleline(&mut model.editable_tether_settings.username);
        });
        ui.horizontal(|ui| {
            ui.label("Password");
            ui.text_edit_singleline(&mut model.editable_tether_settings.password);
        });
        if ui.button("Apply").clicked() {
            model.editable_tether_settings.is_editing = false;
            info!("Re(creating) Tether Agent with new settings...");

            attempt_new_tether_connection(model);
        }
    } else {
        ui.label(model.tether_agent.broker_uri());

        if ui.button("Edit").clicked() {
            model.editable_tether_settings.is_editing = true;
        }
    }

    if model.tether_agent.is_connected() {
        ui.label(RichText::new("Connected ☑").color(Color32::GREEN));
    } else {
        ui.label(RichText::new("Not connected ✖").color(Color32::RED));
        if ui.button("Connect").clicked() {
            attempt_new_tether_connection(model);
        }
    }

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Role");
        if ui
            .text_edit_singleline(&mut model.editable_tether_settings.role)
            .changed()
        {
            model
                .tether_agent
                .set_role(&model.editable_tether_settings.role);
            // model.prepare_next_entry();
        }
    });
    ui.horizontal(|ui| {
        ui.label("ID or Group");
        if ui
            .text_edit_singleline(&mut model.editable_tether_settings.id)
            .changed()
        {
            model
                .tether_agent
                .set_id(&model.editable_tether_settings.id);
            // model.prepare_next_entry();
        }
    });

    standard_spacer(ui);
    ui.separator();
    ui.heading("Insights");
    ui.checkbox(&mut model.continuous_mode, "Continuous mode")
        .on_hover_text("Message log will update immediately; CPU usage may be higher");
    ui.collapsing(format!("Topics x{}", model.insights.topics().len()), |ui| {
        for t in model.insights.topics() {
            ui.small(t);
        }
    });
    ui.collapsing(
        format!("Plug Names x{}", model.insights.plugs().len()),
        |ui| {
            for t in model.insights.plugs() {
                ui.small(t);
            }
        },
    );
    ui.collapsing(
        format!("Agent Roles x{}", model.insights.roles().len()),
        |ui| {
            for t in model.insights.roles() {
                ui.small(t);
            }
        },
    );
    ui.collapsing(
        format!("Agent IDs (groups) x{}", model.insights.ids().len()),
        |ui| {
            for t in model.insights.ids() {
                ui.small(t);
            }
        },
    );

    standard_spacer(ui);
    ui.separator();
    ui.heading(format!("Messages x{}", model.insights.message_count()));
    if model.insights.message_log().is_empty() {
        ui.small("0 messages received");
    }
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for (topic, json) in model.insights.message_log().iter().rev() {
                ui.colored_label(Color32::LIGHT_BLUE, topic);
                ui.label(json);
            }
        });
}

pub fn render(ctx: &egui::Context, model: &mut Model) {
    egui::SidePanel::left("General")
        .min_width(256.0)
        .show(ctx, |ui| {
            general_agent_area(ui, model);
        });

    egui::SidePanel::right("Available Widgets")
        .min_width(128.)
        .show(ctx, |ui| {
            ui.heading("Available Widgets");

            standard_spacer(ui);

            available_widgets(ui, model);
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        widgets_in_use(ctx, ui, model);
    });
}
