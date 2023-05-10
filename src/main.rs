#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clap::Parser;
use settings::Cli;
use std::fs;
use ui::{available_widgets, standard_spacer};

extern crate rmp_serde;
extern crate rmpv;
extern crate serde_json;

use eframe::egui;
use egui::{Color32, RichText, Slider};
use env_logger::Env;
use insights::Insights;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;
use widgets::{BoolWidget, ColourRGBA8, ColourWidget, Common, NumberWidget, Widget};

mod insights;
mod settings;
mod ui;
mod widgets;

fn main() -> Result<(), eframe::Error> {
    let cli = Cli::parse();

    // Initialize the logger from the environment
    env_logger::Builder::from_env(Env::default().default_filter_or(&cli.log_level)).init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 960.0)),
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
    FloatNumber(NumberWidget<f64>),
    WholeNumber(NumberWidget<i64>),
    Colour(ColourWidget<ColourRGBA8>),
    Bool(BoolWidget),
}
pub struct Model {
    next_widget: Common,
    next_range: (f32, f32),
    use_custom_topic: bool,
    next_topic: String,
    agent_role: String,
    agent_id: String,
    widgets: Vec<WidgetEntry>,
    queue: Vec<QueueItem>,
    tether_agent: TetherAgent,
    insights: Insights,
    continuous_mode: bool,
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
        let cli = Cli::parse();

        let tether_agent = TetherAgent::new("gui", None, Some(cli.tether_host));
        let (role, id) = tether_agent.description();
        let next_widget = create_next_widget(0, &tether_agent);
        let next_topic = next_widget.plug.topic.clone();

        if cli.tether_disable {
            warn!("Tether disabled; please connect manually if required");
        } else {
            tether_agent.connect();
        }

        let widgets = load_widgets_from_disk();

        Self {
            next_widget,
            next_range: (0., 1.0),
            use_custom_topic: false,
            next_topic,
            agent_role: role.into(),
            agent_id: id.into(),
            widgets,
            queue: Vec::new(),
            insights: Insights::new(&tether_agent, cli.tether_disable),
            tether_agent,
            continuous_mode: true,
        }
    }
}

fn load_widgets_from_disk() -> Vec<WidgetEntry> {
    let text = fs::read_to_string("./widgets.json");
    let widgets = match text {
        Ok(d) => {
            info!("Found widget data file; parsing...");
            let widgets =
                serde_json::from_str::<Vec<WidgetEntry>>(&d).expect("failed to parse widget list");
            info!("... loaded {} widgets OK", widgets.len());
            // TODO: optionally "broadcast" all values from loaded Widgets
            widgets
        }
        Err(e) => {
            error!("Failed to load widgets from disk: {:?}", e);
            Vec::new()
        }
    };
    widgets
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
                let shortened_name = String::from(self.next_widget.name.replace(' ', "_").trim());
                self.next_widget.plug.name = shortened_name.clone();
                if !self.use_custom_topic {
                    let (role, id) = self.tether_agent.description();
                    self.next_topic = format!("{role}/{id}/{}", shortened_name);
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
                && !self.use_custom_topic
            {
                let (role, id) = self.tether_agent.description();
                let plug_name = self.next_widget.plug.name.clone();
                self.next_topic = format!("{role}/{id}/{plug_name}");
            }
        });
        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut self.use_custom_topic, "Use custom topic")
                .changed()
                && !self.use_custom_topic
            {
                let (role, id) = self.tether_agent.description();
                let plug_name = self.next_widget.plug.name.clone();
                self.next_topic = format!("{role}/{id}/{plug_name}");
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
        if self.insights.update(&self.tether_agent) || self.continuous_mode {
            ctx.request_repaint();
        }

        while let Some(q) = self.queue.pop() {
            match q {
                QueueItem::Remove(index) => {
                    self.widgets.remove(index);
                }
            }
        }

        egui::SidePanel::left("Settings").show(ctx, |ui| {
            ui.heading("Tether Agent");

            standard_spacer(ui);
            ui.separator();
            ui.heading("Load/Save");
            ui.horizontal(|ui| {
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
                if ui.button("Load").clicked() {
                    self.widgets = load_widgets_from_disk();
                }
                if ui.button("Clear").clicked() {
                    self.widgets.clear();
                }
            });

            standard_spacer(ui);
            ui.separator();
            ui.heading("Agent");

            ui.label(self.tether_agent.broker_uri());

            if self.tether_agent.is_connected() {
                ui.label(RichText::new("Connected ☑").color(Color32::GREEN));
            } else {
                ui.label(RichText::new("Not connected ✖").color(Color32::RED));
                if ui.button("Connect").clicked() {
                    self.tether_agent.connect();
                    self.insights = Insights::new(&self.tether_agent, false);
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

            standard_spacer(ui);
            ui.separator();
            ui.heading("Insights");
            ui.checkbox(&mut self.continuous_mode, "Continuous mode");
            ui.label(format!("Topics x{}", self.insights.topics().len()));
            for t in self.insights.topics() {
                ui.small(t);
            }
            ui.separator();
            ui.label(format!("Plug Names x{}", self.insights.plugs().len()));
            for t in self.insights.plugs() {
                ui.small(t);
            }
            ui.separator();
            ui.label(format!("Agent Roles x{}", self.insights.roles().len()));
            for t in self.insights.roles() {
                ui.small(t);
            }
            ui.separator();
            ui.label(format!("Agent IDs (groups) x{}", self.insights.ids().len()));
            for t in self.insights.ids() {
                ui.small(t);
            }

            standard_spacer(ui);
            ui.separator();
            ui.heading(format!("Messages x{}", self.insights.message_count()));
            if self.insights.message_log().is_empty() {
                ui.small("0 messages received");
            }
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (topic, json) in self.insights.message_log().iter().rev() {
                        ui.colored_label(Color32::LIGHT_BLUE, topic);
                        ui.label(json);
                    }
                });
        });

        egui::SidePanel::right("Custom UI")
            .min_width(512.)
            .show(ctx, |ui| {
                ui.heading("Entries");

                standard_spacer(ui);
                // TODO: use grid

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for (i, entry) in self.widgets.iter_mut().enumerate() {
                            ui.separator();

                            match entry {
                                WidgetEntry::FloatNumber(e) => {
                                    let (min, max) = e.range();
                                    let heading =
                                        format!("Number: {} ({}..={})", e.common().name, min, max);
                                    entry_heading(ui, heading);
                                    if ui
                                        .add(
                                            Slider::new(e.value_mut(), min..=max)
                                                .clamp_to_range(false),
                                        )
                                        .changed()
                                    {
                                        if self.tether_agent.is_connected() {
                                            self.tether_agent
                                                .encode_and_publish(&e.common().plug, e.value())
                                                .expect("Failed to send number");
                                        }
                                    };
                                    entry_footer(ui, e);
                                }
                                WidgetEntry::WholeNumber(e) => {
                                    let (min, max) = e.range();
                                    let heading =
                                        format!("Number: {} ({}..={})", e.common().name, min, max);
                                    entry_heading(ui, heading);
                                    if ui
                                        .add(
                                            Slider::new(e.value_mut(), min..=max)
                                                .clamp_to_range(false),
                                        )
                                        .changed()
                                    {
                                        if self.tether_agent.is_connected() {
                                            self.tether_agent
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
                                        if self.tether_agent.is_connected() {
                                            self.tether_agent
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
                                        if self.tether_agent.is_connected() {
                                            self.tether_agent
                                                .encode_and_publish(&e.common().plug, e.value())
                                                .expect("Failed to send boolean");
                                        }
                                    }
                                    entry_footer(ui, e);
                                }
                            }

                            if ui.button("❌ Remove").clicked() {
                                self.queue.push(QueueItem::Remove(i));
                            }

                            standard_spacer(ui);
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |_ui| {
            available_widgets(ctx, self);
        });
    }
}

fn entry_heading(ui: &mut egui::Ui, heading: String) {
    ui.label(RichText::new(heading).color(Color32::WHITE));
}

fn entry_footer<T>(ui: &mut egui::Ui, entry: &impl Widget<T>) {
    ui.small(&entry.common().description);
    ui.label(
        RichText::new(&format!("Topic: {}", entry.common().plug.topic)).color(Color32::LIGHT_BLUE),
    );
}
