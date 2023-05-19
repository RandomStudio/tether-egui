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
use egui::{plot::PlotPoint, Color32, RichText, Slider, Ui};
use log::{error, info};
use serde_json::Value;

use crate::{Model, WidgetEntry};

const PLOT_SIZE: f32 = 200.0;
pub const ENTRY_GRID_WIDTH: f32 = 200.;

pub fn standard_spacer(ui: &mut egui::Ui) {
    ui.add_space(16.);
}

pub fn common_in_use_heading<T>(ui: &mut egui::Ui, entry: &mut impl CustomWidget<T>) {
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

pub fn entry_topic<T>(ui: &mut egui::Ui, entry: &impl CustomWidget<T>) {
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

pub fn widget_entries(ctx: &egui::Context, ui: &mut Ui, model: &mut Model) {
    // ui.checkbox(&mut model.auto_send, "Auto send")
    //     .on_hover_text(
    //     "Trigger messages on any value change, where possible, instead of waiting for Send button",
    // );
    // standard_spacer(ui);

    let widgets = &mut model.widgets;

    for (i, entry) in widgets.iter_mut().enumerate() {
        match entry {
            WidgetEntry::FloatNumber(e) => {
                // egui::Grid::new(format!("grid{}", i))
                //     .num_columns(3)
                //     .striped(true)
                //     .min_col_width(ENTRY_GRID_WIDTH)
                //     .show(ui, |ui| {
                //         // Col1
                //         entry_heading(ui, e);

                //         // Col 2
                //         let res = ui.vertical(|ui| {
                //             let (min, max) = e.range();
                //             let s =
                //                 ui.add(Slider::new(e.value_mut(), min..=max).clamp_to_range(false));
                //             ui.small(format!("Range: {}-{}", min, max));
                //             s
                //         });

                //         // Col 3
                //         ui.vertical(|ui| {
                //             if model.tether_agent.is_connected() && ui.button("Send").clicked()
                //                 || res.inner.changed() && model.auto_send
                //             {
                //                 // println!("changed? {:?}", res.inner);
                //                 model
                //                     .tether_agent
                //                     .encode_and_publish(&e.common().plug, e.value())
                //                     .expect("Failed to send number");
                //             }
                //             entry_topic(ui, e);
                //         })
                //     });

                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }
            }
            WidgetEntry::WholeNumber(e) => {
                // egui::Grid::new(format!("grid{}", i))
                //     .num_columns(3)
                //     .striped(true)
                //     .min_col_width(ENTRY_GRID_WIDTH)
                //     .show(ui, |ui| {
                //         // Col1
                //         entry_heading(ui, e);

                //         // Col 2
                //         let res = ui.vertical(|ui| {
                //             let (min, max) = e.range();
                //             let s =
                //                 ui.add(Slider::new(e.value_mut(), min..=max).clamp_to_range(false));
                //             ui.small(format!("Range: {}-{}", min, max));
                //             s
                //         });

                //         // Col 3
                //         ui.vertical(|ui| {
                //             if model.tether_agent.is_connected() && ui.button("Send").clicked()
                //                 || res.inner.changed() && model.auto_send
                //             {
                //                 // println!("changed? {:?}", res.inner);
                //                 model
                //                     .tether_agent
                //                     .encode_and_publish(&e.common().plug, e.value())
                //                     .expect("Failed to send number");
                //             }
                //             entry_topic(ui, e);
                //         })
                //     });

                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }
            }
            WidgetEntry::Colour(e) => {
                // egui::Grid::new(format!("grid{}", i))
                //     .num_columns(3)
                //     .striped(true)
                //     .min_col_width(ENTRY_GRID_WIDTH)
                //     .show(ui, |ui| {
                //         // Col 1
                //         entry_heading(ui, e);

                //         // Col 2
                //         let res = ui.vertical(|ui| {
                //             let color_picker =
                //                 ui.color_edit_button_srgba_unmultiplied(e.value_mut());
                //             let srgba = e.value();
                //             ui.label(format!(
                //                 "sRGBA: {} {} {} {}",
                //                 srgba[0], srgba[1], srgba[2], srgba[3],
                //             ));
                //             color_picker
                //         });

                //         // Col 3
                //         ui.vertical(|ui| {
                //             if model.tether_agent.is_connected() && ui.button("Send").clicked()
                //                 || res.inner.changed() && model.auto_send
                //             {
                //                 model
                //                     .tether_agent
                //                     .encode_and_publish(&e.common().plug, e.value())
                //                     .expect("Failed to send colour")
                //             }
                //             entry_topic(ui, e);
                //         });
                //     });

                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }
            }
            WidgetEntry::Bool(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i);
                }
                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }
            }
            WidgetEntry::Empty(e) => {
                if e.is_edit_mode() {
                    e.render_editing(ctx, i);
                } else {
                    e.render_in_use(ctx, i);
                }
                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }

                // egui::Grid::new(format!("grid{}", i))
                //     .num_columns(3)
                //     .striped(true)
                //     .min_col_width(ENTRY_GRID_WIDTH)
                //     .show(ui, |ui| {
                //         // Col 1
                //         entry_heading(ui, e);

                //         // Col 2
                //         ui.label("Empty message body");

                //         // Col 3
                //         ui.vertical(|ui| {
                //             if model.tether_agent.is_connected() && ui.button("Send").clicked() {
                //                 model
                //                     .tether_agent
                //                     .encode_and_publish(&e.common().plug, e.value())
                //                     .expect("Failed to send boolean");
                //             }
                //             entry_topic(ui, e);
                //         })
                //     });
                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }
            }
            WidgetEntry::Point2D(e) => {
                // egui::Grid::new(format!("grid{}", i))
                //     .num_columns(3)
                //     .striped(true)
                //     .min_col_width(ENTRY_GRID_WIDTH)
                //     .show(ui, |ui| {
                //         // Col 1
                //         entry_heading(ui, e);

                //         // Col 2
                //         let res = ui.vertical(|ui| {
                //             let plotter = egui::plot::Plot::new("tracking_plot")
                //                 .width(PLOT_SIZE)
                //                 .height(PLOT_SIZE)
                //                 .data_aspect(1.0)
                //                 .show(ui, |plot_ui| {
                //                     (
                //                         plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
                //                         plot_ui.pointer_coordinate(),
                //                         plot_ui.pointer_coordinate_drag_delta(),
                //                         plot_ui.plot_bounds(),
                //                         plot_ui.plot_hovered(),
                //                     )
                //                 });
                //                 ui.collapsing("Instructions", |ui| {
                //                     ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                //                     ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
                //                     if cfg!(target_arch = "wasm32") {
                //                         ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                //                     } else if cfg!(target_os = "macos") {
                //                         ui.label("Zoom with ctrl / ⌘ + scroll.");
                //                     } else {
                //                         ui.label("Zoom with ctrl + scroll.");
                //                     }
                //                     ui.label("Reset view with double-click.");
                //                 });
                //                 plotter
                //         });

                //         let (
                //             _screen_pos,
                //             pointer_coordinate,
                //             _pointer_coordinate_drag_delta,
                //             _bounds,
                //             hovered,
                //         ) = res.inner.inner;

                //         // Col 3
                //         ui.vertical(|ui| {
                //             if model.tether_agent.is_connected() && ui.button("Send").clicked()
                //                 || hovered && model.auto_send
                //             {
                //                 if let Some(c) = pointer_coordinate {
                //                     // println!("Pointer coordinates: {:?}", c)
                //                     let PlotPoint { x, y } = c;
                //                     let p = [x, y];
                //                     model
                //                         .tether_agent
                //                         .encode_and_publish(&e.common().plug, p)
                //                         .expect("Failed to send Point2D message");
                //                 }
                //             }

                //             entry_topic(ui, e);
                //         });
                //     });
                // if entry_remove(ui) {
                //     model.queue.push(QueueItem::Remove(i));
                // }
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
            false,
            &model.tether_agent,
        )));
    }
    if ui.button("Empty").clicked() {
        model.widgets.push(WidgetEntry::Empty(EmptyWidget::new(
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
            &model.tether_agent,
        )));
    }
    // egui::Window::new("Floating-Point Number")
    //     .default_open(false)
    //     .show(ctx, |ui| {
    //         egui::Grid::new("my_grid")
    //             .num_columns(2)
    //             .striped(true)
    //             .show(ui, |ui| {
    //                 common_widget_values(ui, model);

    //                 standard_spacer(ui);

    //                 number_widget_range(ui, model, 1.);

    //                 if ui.button("✚ Add").clicked() {
    //                     model
    //                         .widgets
    //                         .push(WidgetEntry::FloatNumber(NumberWidget::new(
    //                             &model.next_widget.name,
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
    //                             0.,
    //                             model.next_range.0.into()..=model.next_range.1.into(),
    //                             &model.tether_agent,
    //                         )));
    //                     model.prepare_next_entry();
    //                 }
    //             });
    //     });

    // egui::Window::new("Whole Number")
    //     .default_open(false)
    //     .show(ctx, |ui| {
    //         egui::Grid::new("my_grid")
    //             .num_columns(2)
    //             .striped(true)
    //             .show(ui, |ui| {
    //                 common_widget_values(ui, model);

    //                 standard_spacer(ui);

    //                 number_widget_range(ui, model, 100.);

    //                 if ui.button("✚ Add").clicked() {
    //                     let min = model.next_range.0 as i64;
    //                     let max = model.next_range.1 as i64;
    //                     model
    //                         .widgets
    //                         .push(WidgetEntry::WholeNumber(NumberWidget::new(
    //                             &model.next_widget.name,
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
    //                             0,
    //                             min..=max,
    //                             &model.tether_agent,
    //                         )));
    //                     model.prepare_next_entry();
    //                 }
    //             });
    //     });

    // egui::Window::new("Colour")
    //     .default_open(false)
    //     .show(ctx, |ui| {
    //         egui::Grid::new("my_grid")
    //             .num_columns(2)
    //             .striped(true)
    //             .show(ui, |ui| {
    //                 common_widget_values(ui, model);
    //                 if ui.button("✚ Add").clicked() {
    //                     model.widgets.push(WidgetEntry::Colour(ColourWidget::new(
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
    //                         [255, 255, 255, 255],
    //                         &model.tether_agent,
    //                     )));
    //                     model.prepare_next_entry();
    //                 }
    //             });
    //     });

    // egui::Window::new("Boolean")
    //     .default_open(false)
    //     .show(ctx, |ui| {
    //         egui::Grid::new("my_grid")
    //             .num_columns(2)
    //             .striped(true)
    //             .show(ui, |ui| {
    //                 common_widget_values(ui, model);
    //                 if ui.button("✚ Add").clicked() {
    //                     model.widgets.push(WidgetEntry::Bool(BoolWidget::new(
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
    //                         false,
    //                         &model.tether_agent,
    //                     )));
    //                     model.prepare_next_entry();
    //                 }
    //             });
    //     });

    // egui::Window::new("Empty Message")
    //     .default_open(false)
    //     .show(ctx, |ui| {
    //         egui::Grid::new("my_grid")
    //             .num_columns(2)
    //             .striped(true)
    //             .show(ui, |ui| {
    //                 common_widget_values(ui, model);
    //                 if ui.button("✚ Add").clicked() {
    //                     model.widgets.push(WidgetEntry::Empty(EmptyWidget::new(
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

pub fn common_editable_values<T>(ui: &mut egui::Ui, entry: &mut impl CustomWidget<T>) {
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
