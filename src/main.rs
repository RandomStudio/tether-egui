#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;

use eframe::egui;
use egui::Slider;
use env_logger::Env;
use log::{error, info};
use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;
use widgets::{ColourWidget, Common, NumberWidget, Widget};

mod widgets;

fn main() -> Result<(), eframe::Error> {
    // Initialize the logger from the environment
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 960.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Tether egui UI Builder",
        options,
        Box::new(|_cc| Box::<Model>::default()),
    )
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

enum WidgetEntry {
    Number(NumberWidget),
    Colour(ColourWidget),
}
struct Model {
    next_widget: Common,
    use_custom_topic: bool,
    next_topic: String,
    agent_role: String,
    agent_id: String,
    widgets: Vec<WidgetEntry>,
    queue: Vec<QueueItem>,
    tether_agent: TetherAgent,
}

fn get_next_name(count: usize) -> String {
    format!("plug{}", count + 1)
}

fn create_next_widget(index: usize, agent: &TetherAgent) -> Common {
    let default_name = get_next_name(index);
    Common::new(&default_name, None, &default_name, None, agent)
}

impl Default for Model {
    fn default() -> Self {
        let tether_agent = TetherAgent::new("gui", None, None);
        let (role, id) = tether_agent.description();
        let next_widget = create_next_widget(0, &tether_agent);
        let next_topic = next_widget.plug.topic.clone();
        tether_agent.connect();
        Self {
            next_widget,
            use_custom_topic: false,
            next_topic,
            agent_role: role.into(),
            agent_id: id.into(),
            widgets: Vec::new(),
            queue: Vec::new(),
            tether_agent,
        }
    }
}

impl Model {
    fn prepare_next_entry(&mut self) {
        self.next_widget = create_next_widget(self.widgets.len(), &self.tether_agent);
        let (role, id) = self.tether_agent.description();
        let plug_name = self.next_widget.plug.name.clone();
        self.next_topic = format!("{role}/{id}/{plug_name}");
        self.use_custom_topic = false;
    }

    fn common_widget_values(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Name");
            if ui
                .text_edit_singleline(&mut self.next_widget.name)
                .changed()
            {
                let shortened_name = String::from(self.next_widget.name.replace(" ", "_").trim());
                self.next_widget.plug.name = shortened_name.clone();
                if !self.use_custom_topic {
                    let (role, id) = self.tether_agent.description();
                    self.next_topic = format!("{role}/{id}/{}", shortened_name.clone());
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Description");
            ui.text_edit_singleline(&mut self.next_widget.description);
        });
        ui.horizontal(|ui| {
            ui.label("Plug Name");
            if ui
                .text_edit_singleline(&mut self.next_widget.plug.name)
                .changed()
            {
                if !self.use_custom_topic {
                    let (role, id) = self.tether_agent.description();
                    let plug_name = self.next_widget.plug.name.clone();
                    self.next_topic = format!("{role}/{id}/{plug_name}");
                }
            }
        });
        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut self.use_custom_topic, "Use custom topic")
                .changed()
            {
                if !self.use_custom_topic {
                    let (role, id) = self.tether_agent.description();
                    let plug_name = self.next_widget.plug.name.clone();
                    self.next_topic = format!("{role}/{id}/{plug_name}");
                }
            }
        });
        ui.add_enabled_ui(self.use_custom_topic, |ui| {
            ui.horizontal(|ui| {
                ui.label("Topic");
                ui.text_edit_singleline(&mut self.next_topic);
            });
        });
    }
}

enum QueueItem {
    Remove(usize),
}

impl eframe::App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Some(q) = self.queue.pop() {
            match q {
                QueueItem::Remove(index) => {
                    self.widgets.remove(index);
                }
            }
        }

        egui::SidePanel::right("Custom UI")
            .min_width(512.)
            .show(ctx, |ui| {
                ui.heading("Entries");
                for (i, entry) in self.widgets.iter_mut().enumerate() {
                    match entry {
                        WidgetEntry::Number(e) => {
                            ui.label(&format!("Number: {}", e.common().name));
                            let (min, max) = e.range();
                            if ui.add(Slider::new(e.value_mut(), min..=max)).changed() {
                                self.tether_agent
                                    .encode_and_publish(&e.common().plug, e.value())
                                    .expect("Failed to send number");
                            };
                            // ui.text_edit_singleline(&mut e.common().topic(&self.tether_agent));
                            ui.label(&format!("Topic: {}", e.common().plug.topic));
                        }
                        WidgetEntry::Colour(e) => {
                            ui.label(&format!("Colour: {}", e.common().name));
                            if ui
                                .color_edit_button_srgba_unmultiplied(e.value_mut())
                                .changed()
                            {
                                self.tether_agent
                                    .encode_and_publish(&e.common().plug, e.value())
                                    .expect("Failed to send colour")
                            };
                            let srgba = e.value();
                            ui.label(format!(
                                "sRGBA: {} {} {} {}",
                                srgba[0], srgba[1], srgba[2], srgba[3],
                            ));
                            ui.small(&e.common().description);
                            ui.label(&format!("Topic: {}", e.common().plug.topic));
                        }
                    }

                    if ui.button("remove").clicked() {
                        self.queue.push(QueueItem::Remove(i));
                    }

                    ui.separator();
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("UI builder");

            if ui.button("Save").clicked() {
                let text = serde_json::to_string_pretty(&self.widgets)
                    .expect("failed to serialise widget data");
                match fs::write("./widgets.json", text) {
                    Ok(()) => {
                        info!("Saved OK");
                    }
                    Err(e) => {
                        error!("Error saving to disk: {:?}", e);
                    }
                }
            }

            ui.separator();

            ui.collapsing("Agent", |ui| {
                if self.tether_agent.is_connected() {
                    ui.heading("Connected ☑");
                } else {
                    ui.heading("Not connected ✖");
                    if ui.button("Connect").clicked() {
                        self.tether_agent.connect();
                    }
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Role");
                    if ui.text_edit_singleline(&mut self.agent_role).changed() {
                        self.tether_agent.set_role(&self.agent_role);
                        self.prepare_next_entry();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("ID or Group");
                    if ui.text_edit_singleline(&mut self.agent_id).changed() {
                        self.tether_agent.set_id(&self.agent_id);
                        self.prepare_next_entry();
                    }
                });
            });

            ui.separator();

            ui.heading("Number");
            self.common_widget_values(ui);
            if ui.button("Add").clicked() {
                self.widgets.push(WidgetEntry::Number(NumberWidget::new(
                    &self.next_widget.name,
                    {
                        if self.next_widget.description == "" {
                            None
                        } else {
                            Some(&self.next_widget.description)
                        }
                    },
                    &self.next_widget.plug.name,
                    {
                        if self.use_custom_topic {
                            Some(&self.next_topic)
                        } else {
                            None
                        }
                    },
                    0.,
                    None,
                    &self.tether_agent,
                )));
                self.prepare_next_entry();
            }

            ui.separator();

            ui.heading("Colours");
            self.common_widget_values(ui);
            if ui.button("Add").clicked() {
                self.widgets.push(WidgetEntry::Colour(ColourWidget::new(
                    self.next_widget.name.as_str(),
                    {
                        if self.next_widget.description == "" {
                            None
                        } else {
                            Some(&self.next_widget.description)
                        }
                    },
                    &self.next_widget.plug.name,
                    {
                        if self.use_custom_topic {
                            Some(&self.next_topic)
                        } else {
                            None
                        }
                    },
                    (255, 255, 255, 255),
                    &self.tether_agent,
                )));
                self.prepare_next_entry();
            }
        });
    }
}
