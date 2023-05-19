use std::fs;

use crate::{
    insights::Insights,
    load_widgets_from_disk,
    widgets::{
        boolean::BoolWidget, colours::ColourWidget, empty::EmptyWidget, generic::GenericJSONWidget,
        numbers::NumberWidget, point::Point2DWidget, Common, CustomWidget, View,
    },
    QueueItem,
};
use egui::{plot::PlotPoint, Color32, Response, RichText, Slider, Ui};
use log::{error, info};
use serde::Serialize;
use serde_json::Value;
use tether_agent::TetherAgent;

use crate::{Model, WidgetEntry};

pub const ENTRY_GRID_WIDTH: f32 = 200.;

pub fn standard_spacer(ui: &mut egui::Ui) {
    ui.add_space(16.);
}

pub fn common_in_use_heading<T: Serialize>(ui: &mut egui::Ui, entry: &mut impl CustomWidget<T>) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(&entry.common().name)
                .color(Color32::WHITE)
                .size(18.),
        );
        if ui.button("edit").clicked() {
            entry.common_mut().set_edit_mode(true);
        }
    });
    ui.small(&entry.common().description);
    ui.separator();
}

pub fn common_save_button<T: Serialize>(ui: &mut egui::Ui, entry: &mut impl CustomWidget<T>) {
    if ui.button("Save").clicked() {
        entry.common_mut().set_edit_mode(false);
    }
}

pub fn common_send_button<T: Serialize>(
    ui: &mut egui::Ui,
    entry: &mut impl CustomWidget<T>,
) -> Response {
    let res = ui.horizontal(|ui| {
        let res = ui.button("Send");
        entry_topic(ui, entry);
        res
    });
    res.inner
}

pub fn common_send<T: Serialize>(entry: &mut impl CustomWidget<T>, tether_agent: &TetherAgent) {
    tether_agent
        .encode_and_publish(&entry.common().plug, &entry.value())
        .expect("Failed to send message");
}

pub fn entry_topic<T: Serialize>(ui: &mut egui::Ui, entry: &impl CustomWidget<T>) {
    ui.label(
        RichText::new(format!("Topic: {}", entry.common().plug.topic)).color(Color32::LIGHT_BLUE),
    );
}

fn entry_remove(ui: &mut Ui) -> bool {
    let clicked = ui.button("❌ Remove").clicked();
    ui.separator();
    standard_spacer(ui);
    clicked
}

pub fn widgets_in_use(ctx: &egui::Context, ui: &mut Ui, model: &mut Model) {
    // ui.checkbox(&mut model.auto_send, "Auto send")
    //     .on_hover_text(
    //     "Trigger messages on any value change, where possible, instead of waiting for Send button",
    // );
    // standard_spacer(ui);

    let widgets = &mut model.widgets;

    for (i, entry) in widgets.iter_mut().enumerate() {
        match entry {
            WidgetEntry::FloatNumber(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i, &model.tether_agent);
                }
            }
            WidgetEntry::WholeNumber(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i, &model.tether_agent);
                }
            }
            WidgetEntry::Colour(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i, &model.tether_agent);
                }
            }
            WidgetEntry::Bool(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i, &model.tether_agent);
                }
            }
            WidgetEntry::Empty(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i, &model.tether_agent);
                }
            }
            WidgetEntry::Point2D(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i, &model.tether_agent);
                }
            }
            WidgetEntry::Generic(e) => {
                // egui::Grid::new(format!("grid{}", i))
                //     .num_columns(3)
                //     .striped(true)
                //     .min_col_width(ENTRY_GRID_WIDTH)
                //     .show(ui, |ui| {
                //         // Col 1
                //         entry_heading(ui, e);

                //         // Col 2
                //         ui.vertical(|ui| {
                //             if ui.text_edit_multiline(e.value_mut()).changed() {
                //                 if serde_json::from_str::<Value>(e.value()).is_err() {
                //                     model.is_valid_json = false;
                //                 } else {
                //                     model.is_valid_json = true;
                //                 }
                //             }
                //             if model.is_valid_json {
                //                 ui.colored_label(Color32::LIGHT_GREEN, "Valid JSON");
                //             } else {
                //                 ui.colored_label(Color32::RED, "Not valid JSON");
                //             }
                //         });

                //         // Col 3
                //         ui.vertical(|ui| {
                //             if model.tether_agent.is_connected() && ui.button("Send").clicked() {
                //                 if let Ok(json) = serde_json::from_str::<Value>(e.value()) {
                //                     match rmp_serde::to_vec_named(&json) {
                //                         Ok(payload) => model
                //                             .tether_agent
                //                             .publish(&e.common().plug, Some(&payload))
                //                             .expect(
                //                                 "Failed to send Generic JSON (encoded as messagepback) message",
                //                             ),
                //                         Err(e) => {
                //                             error!("Failed to encode MessagePack payload: {}", e);
                //                         }
                //                     }
                //                 }
                //             }
                //             entry_topic(ui, e);
                //         });
                //     });
                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }
            }
        }

        ui.end_row();

        standard_spacer(ui);

        ui.end_row();
    }
}

pub fn available_widgets(ui: &mut egui::Ui, model: &mut Model) {
    if ui.button("Boolean").clicked() {
        model.widgets.push(WidgetEntry::Bool(BoolWidget::new(
            "Boolean Meassage",
            Some("A true or false value"),
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
    }
    if ui.button("Floating Point").clicked() {
        model
            .widgets
            .push(WidgetEntry::FloatNumber(NumberWidget::new(
                "Floating Point Number",
                Some("A single 64-bit floating point number"),
                &model.next_widget.plug.name,
                {
                    if model.use_custom_topic {
                        Some(&model.next_topic)
                    } else {
                        None
                    }
                },
                0.,
                0. ..=1.0,
                &model.tether_agent,
            )));
    }
    if ui.button("Whole Number").clicked() {
        model
            .widgets
            .push(WidgetEntry::WholeNumber(NumberWidget::new(
                "Whole Number",
                Some("A single 64-bit whole number"),
                &model.next_widget.plug.name,
                {
                    if model.use_custom_topic {
                        Some(&model.next_topic)
                    } else {
                        None
                    }
                },
                0,
                0..=100,
                &model.tether_agent,
            )));
    }
    if ui.button("Point2D").clicked() {
        model.widgets.push(WidgetEntry::Point2D(Point2DWidget::new(
            "Point2D",
            Some("X and Y values"),
            &model.next_widget.plug.name,
            {
                if model.use_custom_topic {
                    Some(&model.next_topic)
                } else {
                    None
                }
            },
            &model.tether_agent,
        )))
    }

    // egui::Window::new("Point2D")
    //     .default_open(false)
    //     .show(ctx, |ui| {
    //         egui::Grid::new("my_grid")
    //             .num_columns(2)
    //             .striped(true)
    //             .show(ui, |ui| {
    //                 common_widget_values(ui, model);
    //                 if ui.button("✚ Add").clicked() {
    //                     model.widgets.push(WidgetEntry::Point2D(Point2DWidget::new(
    //                         model.next_widget.name.as_str(),
    //                         {
    //                             if model.next_widget.description.is_empty() {
    //                                 None
    //                             } else {
    //                                 Some(&model.next_widget.description)
    //                             }
    //                         },
    //                         &model.next_widget.plug.name,
    //                         {
    //                             if model.use_custom_topic {
    //                                 Some(&model.next_topic)
    //                             } else {
    //                                 None
    //                             }
    //                         },
    //                         &model.tether_agent,
    //                     )));
    //                     model.prepare_next_entry();
    //                 }
    //             });
    //     });

    // egui::Window::new("Generic JSON data")
    //     .default_open(false)
    //     .show(ctx, |ui| {
    //         egui::Grid::new("my_grid")
    //             .num_columns(2)
    //             .striped(true)
    //             .show(ui, |ui| {
    //                 common_widget_values(ui, model);
    //                 if ui.button("✚ Add").clicked() {
    //                     model
    //                         .widgets
    //                         .push(WidgetEntry::Generic(GenericJSONWidget::new(
    //                             model.next_widget.name.as_str(),
    //                             {
    //                                 if model.next_widget.description.is_empty() {
    //                                     None
    //                                 } else {
    //                                     Some(&model.next_widget.description)
    //                                 }
    //                             },
    //                             &model.next_widget.plug.name,
    //                             {
    //                                 if model.use_custom_topic {
    //                                     Some(&model.next_topic)
    //                                 } else {
    //                                     None
    //                                 }
    //                             },
    //                             &model.tether_agent,
    //                         )));
    //                     model.prepare_next_entry();
    //                 }
    //             });
    //     });
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

pub fn general_agent_area(ui: &mut Ui, model: &mut Model) {
    ui.heading("Tether Agent");

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
            // TODO: need to use username, password, monitor topic if set!
            match model.tether_agent.connect(None, None) {
                Ok(()) => {
                    model.insights = Insights::new(&model.tether_agent, "#");
                }
                Err(e) => {
                    error!("Tether Agent failed to connect: {}", e);
                }
            }
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

pub fn common_editable_values<T: Serialize>(ui: &mut egui::Ui, entry: &mut impl CustomWidget<T>) {
    ui.label("name");
    if ui
        .text_edit_singleline(&mut entry.common_mut().name)
        .changed()
    {
        let shortened_name = String::from(entry.common().name.replace(' ', "_").trim());
        entry.common_mut().plug.name = shortened_name.clone();
        // if !common.use_custom_topic {
        //     let (role, id) = model.tether_agent.description();
        //     common. = format!("{role}/{id}/{}", shortened_name);
        // }
    }

    ui.label("Description");
    ui.text_edit_multiline(&mut entry.common_mut().description);

    ui.label("Plug Name");
    if ui
        .text_edit_singleline(&mut entry.common_mut().plug.name)
        .changed()
        && !entry.common().use_custom_topic
    {
        // let (role, id) = model.tether_agent.description();
        // let plug_name = model.next_widget.plug.name.clone();
        // model.next_topic = format!("{role}/{id}/{plug_name}");
    }
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
