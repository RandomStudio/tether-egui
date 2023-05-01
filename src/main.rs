#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::net::Ipv4Addr;

use eframe::egui;
use egui::Slider;
use tether::TetherAgent;
use tweaks::{ColourTweak, NumberTweak, Tweak};

mod tweaks;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 960.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<Model>::default()),
    )
}

enum TweakEntry {
    Number(NumberTweak),
    Colour(ColourTweak),
}
struct Model {
    next_name: String,
    next_description: String,
    next_plug_name: String,
    agent_role: String,
    agent_id: String,
    tweaks: Vec<TweakEntry>,
    queue: Vec<QueueItem>,
    tether: TetherAgent,
}

impl Model {
    fn prepare_next_entry(&mut self) {
        self.next_name = get_next_name(self.tweaks.len());
        self.next_description = String::from("");
        self.next_plug_name = self.next_name.clone();
    }
}

fn get_next_name(count: usize) -> String {
    format!("plug{}", count + 1)
}

impl Default for Model {
    fn default() -> Self {
        let next_name = get_next_name(0);
        let next_description = String::from("");
        let next_plug_name = next_name.clone();
        let tether = TetherAgent::new(
            std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            "tweaks",
            None,
        );
        tether.connect();
        Self {
            next_name,
            next_description,
            next_plug_name,
            agent_role: "dummy".into(),
            agent_id: "any".into(),
            tweaks: Vec::new(),
            queue: Vec::new(),
            tether,
        }
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
                    self.tweaks.remove(index);
                }
            }
        }

        egui::SidePanel::right("Custom UI")
            .min_width(512.)
            .show(ctx, |ui| {
                ui.heading("Entries");
                for (i, entry) in self.tweaks.iter_mut().enumerate() {
                    match entry {
                        TweakEntry::Number(e) => {
                            ui.label(&format!("Number: {}", e.common().name));
                            let (min, max) = e.range();
                            ui.add(Slider::new(e.value_mut(), min..=max));
                            ui.label(&format!(
                                "Topic: {}",
                                e.common().topic(&self.agent_role, &self.agent_id)
                            ));
                            if ui.button("Send").clicked() {
                                self.tether
                                    .publish(
                                        &e.value(),
                                        &e.topic(&self.agent_role, &self.agent_id),
                                        None,
                                    )
                                    .expect("Failed to send");
                            }
                        }
                        TweakEntry::Colour(e) => {
                            ui.label(&format!("Colour: {}", e.common().name));
                            ui.color_edit_button_srgba_unmultiplied(e.value_mut());
                            let srgba = e.value();
                            ui.label(format!(
                                "sRGBA: {} {} {} {}",
                                srgba[0], srgba[1], srgba[2], srgba[3],
                            ));
                            ui.small(&e.common().description);
                            ui.label(&format!(
                                "Topic: {}",
                                e.common().topic(&self.agent_role, &self.agent_id)
                            ));
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

            ui.collapsing("Agent", |ui| {
                if self.tether.is_connected() {
                    ui.heading("Connected ☑");
                } else {
                    ui.heading("Not connected ✖");
                    if ui.button("Connect").clicked() {
                        self.tether.connect();
                    }
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Role");
                    if ui.text_edit_singleline(&mut self.agent_role).changed() {
                        self.tether.set_role(&self.agent_role);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("ID or Group");
                    if ui.text_edit_singleline(&mut self.agent_id).changed() {
                        self.tether.set_id(&self.agent_id);
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Name");
                if ui.text_edit_singleline(&mut self.next_name).changed() {
                    self.next_plug_name = self.next_name.clone();
                };
            });
            ui.horizontal(|ui| {
                ui.label("Description");
                ui.text_edit_singleline(&mut self.next_description);
            });
            ui.horizontal(|ui| {
                ui.label("Plug Name");
                ui.text_edit_singleline(&mut self.next_plug_name);
            });
            if ui.button("Add Number value").clicked() {
                self.tweaks.push(TweakEntry::Number(NumberTweak::new(
                    self.next_name.as_str(),
                    {
                        if self.next_description == "" {
                            None
                        } else {
                            Some(self.next_description.as_str())
                        }
                    },
                    Some(self.next_plug_name.as_str()),
                    0.,
                    None,
                )));
                self.prepare_next_entry();
            }
            if ui.button("Add Colour value").clicked() {
                self.tweaks.push(TweakEntry::Colour(ColourTweak::new(
                    self.next_name.as_str(),
                    {
                        if self.next_description == "" {
                            None
                        } else {
                            Some(self.next_description.as_str())
                        }
                    },
                    Some(self.next_plug_name.as_str()),
                    (255, 255, 255, 255),
                )));
                self.prepare_next_entry();
            }
        });
    }
}
