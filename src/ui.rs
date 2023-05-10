use std::fs;

use crate::{insights::Insights, load_widgets_from_disk, QueueItem, Widget};
use egui::{Color32, RichText, Slider, Ui};
use log::{error, info};

use crate::{
    widgets::{BoolWidget, ColourWidget, NumberWidget},
    Model, WidgetEntry,
};

pub fn standard_spacer(ui: &mut egui::Ui) {
    ui.add_space(16.);
}

pub fn entry_heading(ui: &mut egui::Ui, heading: String) {
    ui.label(RichText::new(heading).color(Color32::WHITE));
}

pub fn entry_footer<T>(ui: &mut egui::Ui, entry: &impl Widget<T>) {
    ui.small(&entry.common().description);
    ui.label(
        RichText::new(&format!("Topic: {}", entry.common().plug.topic)).color(Color32::LIGHT_BLUE),
    );
}

pub fn available_widgets(ctx: &egui::Context, model: &mut Model) {
    egui::Window::new("Floating-Point Number")
        .default_open(false)
        .show(ctx, |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    common_widget_values(ui, model);

                    standard_spacer(ui);

                    number_widget_range(ui, model, 1.);

                    if ui.button("✚ Add").clicked() {
                        model
                            .widgets
                            .push(WidgetEntry::FloatNumber(NumberWidget::new(
                                &model.next_widget.name,
                                {
                                    if model.next_widget.description.is_empty() {
                                        None
                                    } else {
                                        Some(&model.next_widget.description)
                                    }
                                },
                                &model.next_widget.plug.name,
                                {
                                    if model.use_custom_topic {
                                        Some(&model.next_topic)
                                    } else {
                                        None
                                    }
                                },
                                0.,
                                model.next_range.0.into()..=model.next_range.1.into(),
                                &model.tether_agent,
                            )));
                        model.prepare_next_entry();
                    }
                });
        });

    egui::Window::new("Whole Number")
        .default_open(false)
        .show(ctx, |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    common_widget_values(ui, model);

                    standard_spacer(ui);

                    number_widget_range(ui, model, 100.);

                    if ui.button("✚ Add").clicked() {
                        let min = model.next_range.0 as i64;
                        let max = model.next_range.1 as i64;
                        model
                            .widgets
                            .push(WidgetEntry::WholeNumber(NumberWidget::new(
                                &model.next_widget.name,
                                {
                                    if model.next_widget.description.is_empty() {
                                        None
                                    } else {
                                        Some(&model.next_widget.description)
                                    }
                                },
                                &model.next_widget.plug.name,
                                {
                                    if model.use_custom_topic {
                                        Some(&model.next_topic)
                                    } else {
                                        None
                                    }
                                },
                                0,
                                min..=max,
                                &model.tether_agent,
                            )));
                        model.prepare_next_entry();
                    }
                });
        });

    egui::Window::new("Colour")
        .default_open(false)
        .show(ctx, |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    common_widget_values(ui, model);
                    if ui.button("✚ Add").clicked() {
                        model.widgets.push(WidgetEntry::Colour(ColourWidget::new(
                            model.next_widget.name.as_str(),
                            {
                                if model.next_widget.description.is_empty() {
                                    None
                                } else {
                                    Some(&model.next_widget.description)
                                }
                            },
                            &model.next_widget.plug.name,
                            {
                                if model.use_custom_topic {
                                    Some(&model.next_topic)
                                } else {
                                    None
                                }
                            },
                            [255, 255, 255, 255],
                            &model.tether_agent,
                        )));
                        model.prepare_next_entry();
                    }
                });
        });

    egui::Window::new("Boolean")
        .default_open(false)
        .show(ctx, |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    common_widget_values(ui, model);
                    if ui.button("✚ Add").clicked() {
                        model.widgets.push(WidgetEntry::Bool(BoolWidget::new(
                            model.next_widget.name.as_str(),
                            {
                                if model.next_widget.description.is_empty() {
                                    None
                                } else {
                                    Some(&model.next_widget.description)
                                }
                            },
                            &model.next_widget.plug.name,
                            {
                                if model.use_custom_topic {
                                    Some(&model.next_topic)
                                } else {
                                    None
                                }
                            },
                            false,
                            &model.tether_agent,
                        )));
                        model.prepare_next_entry();
                    }
                });
        });
}

fn number_widget_range(ui: &mut Ui, model: &mut Model, default_max: f32) {
    let openness = ui
        .collapsing("Range", |ui| {
            ui.label("Range");
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut model.next_range.0, i16::MIN as f32..=i16::MAX as f32)
                        .text("min"),
                );
                ui.add(
                    egui::Slider::new(&mut model.next_range.1, i16::MIN as f32..=i16::MAX as f32)
                        .text("max"),
                );
                if ui.small_button("Reset").clicked() {
                    model.next_range = (0., default_max);
                }
            });
        })
        .openness;

    if openness > 0. && openness < 1.0 {
        model.next_range = (0., default_max);
    }
    ui.end_row();
}

pub fn widget_entries(ui: &mut Ui, model: &mut Model) {
    for (i, entry) in model.widgets.iter_mut().enumerate() {
        ui.separator();

        match entry {
            WidgetEntry::FloatNumber(e) => {
                let (min, max) = e.range();
                let heading = format!("Number: {} ({}..={})", e.common().name, min, max);
                entry_heading(ui, heading);
                if ui
                    .add(Slider::new(e.value_mut(), min..=max).clamp_to_range(false))
                    .changed()
                {
                    if model.tether_agent.is_connected() {
                        model
                            .tether_agent
                            .encode_and_publish(&e.common().plug, e.value())
                            .expect("Failed to send number");
                    }
                };
                entry_footer(ui, e);
            }
            WidgetEntry::WholeNumber(e) => {
                let (min, max) = e.range();
                let heading = format!("Number: {} ({}..={})", e.common().name, min, max);
                entry_heading(ui, heading);
                if ui
                    .add(Slider::new(e.value_mut(), min..=max).clamp_to_range(false))
                    .changed()
                {
                    if model.tether_agent.is_connected() {
                        model
                            .tether_agent
                            .encode_and_publish(&e.common().plug, e.value())
                            .expect("Failed to send number");
                    }
                };
                entry_footer(ui, e);
            }
            WidgetEntry::Colour(e) => {
                entry_heading(ui, format!("Colour: {}", e.common().name));
                if ui
                    .color_edit_button_srgba_unmultiplied(e.value_mut())
                    .changed()
                {
                    if model.tether_agent.is_connected() {
                        model
                            .tether_agent
                            .encode_and_publish(&e.common().plug, e.value())
                            .expect("Failed to send colour")
                    }
                };
                let srgba = e.value();
                ui.label(format!(
                    "sRGBA: {} {} {} {}",
                    srgba[0], srgba[1], srgba[2], srgba[3],
                ));
                entry_footer(ui, e);
            }
            WidgetEntry::Bool(e) => {
                entry_heading(ui, format!("Boolean: {}", e.common().name));
                let checked = *e.value();
                if ui
                    .checkbox(
                        e.value_mut(),
                        format!("State: {}", {
                            if checked {
                                "TRUE"
                            } else {
                                "FALSE "
                            }
                        }),
                    )
                    .changed()
                {
                    if model.tether_agent.is_connected() {
                        model
                            .tether_agent
                            .encode_and_publish(&e.common().plug, e.value())
                            .expect("Failed to send boolean");
                    }
                }
                entry_footer(ui, e);
            }
        }

        if ui.button("❌ Remove").clicked() {
            model.queue.push(QueueItem::Remove(i));
        }

        standard_spacer(ui);
    }
}

pub fn general_agent_area(ui: &mut Ui, model: &mut Model) {
    ui.heading("Tether Agent");

    standard_spacer(ui);
    ui.separator();
    ui.heading("Load/Save");
    if let Some(json_path) = &model.json_file {
        ui.small(json_path);
    }
    ui.horizontal(|ui| {
        if ui.button("Save").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("text", &["json"])
                .save_file()
            {
                let path_string = path.display().to_string();
                let text = serde_json::to_string_pretty(&model.widgets)
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
                model.widgets =
                    load_widgets_from_disk(&path_string).expect("failed to load widgets");
                model.json_file = Some(path_string);
            }
        }
        if ui.button("Clear").clicked() {
            model.widgets.clear();
            model.json_file = None;
        }
    });

    standard_spacer(ui);
    ui.separator();
    ui.heading("Agent");

    ui.label(model.tether_agent.broker_uri());

    if model.tether_agent.is_connected() {
        ui.label(RichText::new("Connected ☑").color(Color32::GREEN));
    } else {
        ui.label(RichText::new("Not connected ✖").color(Color32::RED));
        if ui.button("Connect").clicked() {
            model.tether_agent.connect();
            model.insights = Insights::new(&model.tether_agent, false);
        }
    }

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Role");
        if ui.text_edit_singleline(&mut model.agent_role).changed() {
            model.tether_agent.set_role(&model.agent_role);
            model.prepare_next_entry();
        }
    });
    ui.horizontal(|ui| {
        ui.label("ID or Group");
        if ui.text_edit_singleline(&mut model.agent_id).changed() {
            model.tether_agent.set_id(&model.agent_id);
            model.prepare_next_entry();
        }
    });

    standard_spacer(ui);
    ui.separator();
    ui.heading("Insights");
    ui.checkbox(&mut model.continuous_mode, "Continuous mode");
    ui.label(format!("Topics x{}", model.insights.topics().len()));
    for t in model.insights.topics() {
        ui.small(t);
    }
    ui.separator();
    ui.label(format!("Plug Names x{}", model.insights.plugs().len()));
    for t in model.insights.plugs() {
        ui.small(t);
    }
    ui.separator();
    ui.label(format!("Agent Roles x{}", model.insights.roles().len()));
    for t in model.insights.roles() {
        ui.small(t);
    }
    ui.separator();
    ui.label(format!(
        "Agent IDs (groups) x{}",
        model.insights.ids().len()
    ));
    for t in model.insights.ids() {
        ui.small(t);
    }

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

pub fn common_widget_values(ui: &mut egui::Ui, model: &mut Model) {
    ui.label("Name");
    if ui
        .text_edit_singleline(&mut model.next_widget.name)
        .changed()
    {
        let shortened_name = String::from(model.next_widget.name.replace(' ', "_").trim());
        model.next_widget.plug.name = shortened_name.clone();
        if !model.use_custom_topic {
            let (role, id) = model.tether_agent.description();
            model.next_topic = format!("{role}/{id}/{}", shortened_name);
        }
    }
    ui.end_row();

    ui.label("Description");
    ui.text_edit_multiline(&mut model.next_widget.description);
    ui.end_row();

    ui.label("Plug Name");
    if ui
        .text_edit_singleline(&mut model.next_widget.plug.name)
        .changed()
        && !model.use_custom_topic
    {
        let (role, id) = model.tether_agent.description();
        let plug_name = model.next_widget.plug.name.clone();
        model.next_topic = format!("{role}/{id}/{plug_name}");
    }
    ui.end_row();

    if ui
        .checkbox(&mut model.use_custom_topic, "Use custom topic")
        .changed()
        && !model.use_custom_topic
    {
        let (role, id) = model.tether_agent.description();
        let plug_name = model.next_widget.plug.name.clone();
        model.next_topic = format!("{role}/{id}/{plug_name}");
    }
    ui.add_enabled_ui(model.use_custom_topic, |ui| {
        ui.text_edit_singleline(&mut model.next_topic);
    });
    ui.end_row();
}
