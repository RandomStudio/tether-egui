#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::Slider;
use tether_agent::TetherAgent;
use tweaks::{ColourTweak, Common, NumberTweak, Tweak};

mod tweaks;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
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

enum TweakEntry {
    Number(NumberTweak),
    Colour(ColourTweak),
}
struct Model {
    next_tweak: Common,
    use_custom_topic: bool,
    next_topic: String,
    agent_role: String,
    agent_id: String,
    tweaks: Vec<TweakEntry>,
    queue: Vec<QueueItem>,
    tether_agent: TetherAgent,
}

// impl Model {
//     fn prepare_next_entry(&mut self) {
//         self.next_name = get_next_name(self.tweaks.len());
//         self.next_description = String::from("");
//         self.next_plug_name = self.next_name.clone();
//     }
// }

fn get_next_name(count: usize) -> String {
    format!("plug{}", count + 1)
}

fn next_tweak(index: usize, agent: &TetherAgent) -> Common {
    let default_name = get_next_name(index);
    Common::new(&default_name, None, &default_name, None, agent)
}

impl Default for Model {
    fn default() -> Self {
        let tether_agent = TetherAgent::new("tweaks", None, None);
        let (role, id) = tether_agent.description();
        let next_tweak = next_tweak(0, &tether_agent);
        let next_topic = next_tweak.plug.topic.clone();
        tether_agent.connect();
        Self {
            next_tweak,
            use_custom_topic: false,
            next_topic,
            agent_role: role.into(),
            agent_id: id.into(),
            tweaks: Vec::new(),
            queue: Vec::new(),
            tether_agent,
        }
    }
}

impl Model {
    fn prepare_next_entry(&mut self) {
        self.next_tweak = next_tweak(self.tweaks.len(), &self.tether_agent);
        // self.next_topic = "".into();
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
                            if ui.add(Slider::new(e.value_mut(), min..=max)).changed() {
                                self.tether_agent
                                    .encode_and_publish(&e.common().plug, e.value())
                                    .expect("Failed to send");
                            };
                            // ui.text_edit_singleline(&mut e.common().topic(&self.tether_agent));
                            ui.label(&format!("Topic: {}", e.common().plug.topic));
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
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("ID or Group");
                    if ui.text_edit_singleline(&mut self.agent_id).changed() {
                        self.tether_agent.set_id(&self.agent_id);
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Name");
                if ui.text_edit_singleline(&mut self.next_tweak.name).changed() {
                    let shortened_name =
                        String::from(self.next_tweak.name.replace(" ", "_").trim());
                    self.next_tweak.plug.name = shortened_name.clone();
                    if !self.use_custom_topic {
                        let (role, id) = self.tether_agent.description();
                        self.next_topic = format!("{role}/{id}/{}", shortened_name.clone());
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Description");
                ui.text_edit_singleline(&mut self.next_tweak.description);
            });
            ui.horizontal(|ui| {
                ui.label("Plug Name");
                if ui
                    .text_edit_singleline(&mut self.next_tweak.plug.name)
                    .changed()
                {
                    if !self.use_custom_topic {
                        let (role, id) = self.tether_agent.description();
                        let plug_name = self.next_tweak.plug.name.clone();
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
                        let plug_name = self.next_tweak.plug.name.clone();
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
            if ui.button("Add Number value").clicked() {
                self.tweaks.push(TweakEntry::Number(NumberTweak::new(
                    &self.next_tweak.name,
                    {
                        if self.next_tweak.description == "" {
                            None
                        } else {
                            Some(&self.next_tweak.description)
                        }
                    },
                    &self.next_tweak.plug.name,
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
            if ui.button("Add Colour value").clicked() {
                self.tweaks.push(TweakEntry::Colour(ColourTweak::new(
                    self.next_tweak.name.as_str(),
                    {
                        if self.next_tweak.description == "" {
                            None
                        } else {
                            Some(&self.next_tweak.description)
                        }
                    },
                    &self.next_tweak.plug.name,
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
