#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::Duration;

use clap::Parser;
use midi_mapping::{toggle_if_midi_note, MidiMessage, MidiSubscriber};
use settings::Cli;
use tether_utils::EditableTetherSettings;
use ui::{available_widgets, general_agent_area, standard_spacer, widgets_in_use};

extern crate rmp_serde;
extern crate rmpv;
extern crate serde_json;

use eframe::egui;
use env_logger::Env;
use insights::Insights;
use log::{error, info, warn};
use tether_agent::{TetherAgent, TetherAgentOptionsBuilder};
use widgets::WidgetEntry;

use crate::{
    midi_mapping::{send_if_midi_note, update_widget_if_controllable},
    project::{Project, TetherSettingsInProject},
    tether_utils::init_new_tether_agent,
    ui::common_send,
};

mod insights;
mod midi_mapping;
mod project;
mod settings;
mod tether_utils;
mod ui;
mod widgets;

fn main() -> Result<(), eframe::Error> {
    let cli = Cli::parse();

    // Initialize the logger from the environment
    env_logger::Builder::from_env(Env::default().default_filter_or(&cli.log_level))
        .filter_module("paho_mqtt", log::LevelFilter::Warn)
        .filter_module("winit", log::LevelFilter::Warn)
        .filter_module("eframe", log::LevelFilter::Warn)
        .init();

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

pub struct Model {
    json_file: Option<String>,
    monitor_topic: String,
    project: Project,
    queue: Vec<QueueItem>,
    insights: Insights,
    midi_handler: MidiSubscriber,
    continuous_mode: bool,
    tether_agent: TetherAgent,
    editable_tether_settings: EditableTetherSettings,
}

impl Default for Model {
    fn default() -> Self {
        let cli = Cli::parse();

        let mut project = Project::default();
        let json_path: String = cli.json_load.unwrap_or(String::from("./project.json"));
        info!("Will attempt to load JSON from {} ...", &json_path);

        let project_loaded = project.load(&json_path);

        let tether_settings = match &project.tether_settings {
            Some(settings) => TetherAgentOptionsBuilder::from(settings),
            None => TetherAgentOptionsBuilder::from(&TetherSettingsInProject::default()),
        };

        // let tether_settings = project.tether_settings.clone().unwrap_or(TetherSettings {
        //     host: cli.tether_host.to_string(),
        //     username: cli.tether_username,
        //     password: cli.tether_password,
        // });

        // let editable_tether_settings = EditableTetherSettings {
        //     is_editing: false,
        //     was_changed: false,
        //     host: cli.tether_host,
        //     username: cli.tether_username.unwrap_or_default(),
        //     password: cli.tether_password.unwrap_or_default()
        // };

        let tether_agent = init_new_tether_agent(&tether_settings);

        if cli.tether_disable {
            warn!("Tether disabled; please connect manually if required");
        } else {
            match tether_agent.connect(&tether_settings) {
                Ok(()) => {
                    info!("Tether Agent connected successfully");
                }
                Err(e) => {
                    error!("Tether Agent failed to connect: {}", e);
                }
            }
        }

        Self {
            json_file: {
                if project_loaded.is_err() {
                    None
                } else {
                    Some(json_path)
                }
            },
            editable_tether_settings: EditableTetherSettings::from(&tether_settings),
            monitor_topic: cli.monitor_topic.clone(),
            project,
            queue: Vec::new(),
            insights: Insights::new(&tether_agent, &cli.monitor_topic),
            midi_handler: MidiSubscriber::new(&tether_agent),
            tether_agent,
            continuous_mode: cli.continuous_mode,
        }
    }
}

// impl Model {
//     fn prepare_next_entry(&mut self) {
//         self.next_widget = create_next_widget(self.widgets.len(), &self.tether_agent);
//         let (role, id) = self.tether_agent.description();
//         let plug_name = self.next_widget.plug.name.clone();
//         self.next_topic = format!("{role}/{id}/{plug_name}");
//         self.use_custom_topic = false;
//     }
// }

enum QueueItem {
    Remove(usize),
}

impl eframe::App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut work_done = false;
        while let Some((plug_name, message)) = &self.tether_agent.check_messages() {
            work_done = true;
            self.insights.update(plug_name, message);
            match self.midi_handler.get_midi_message(plug_name, message) {
                Some(MidiMessage::ControlChange(cc_message)) => {
                    for widget in self.project.widgets.iter_mut() {
                        match widget {
                            WidgetEntry::FloatNumber(e) => {
                                update_widget_if_controllable(e, &cc_message, &self.tether_agent);
                            }
                            WidgetEntry::WholeNumber(e) => {
                                update_widget_if_controllable(e, &cc_message, &self.tether_agent);
                            }
                            _ => {}
                        }
                    }
                }
                Some(MidiMessage::Note(note_message)) => {
                    for widget in self.project.widgets.iter_mut() {
                        match widget {
                            WidgetEntry::Bool(e) => {
                                toggle_if_midi_note(e, &note_message, &self.tether_agent);
                            }
                            WidgetEntry::Empty(e) => {
                                if send_if_midi_note(e, &note_message) {
                                    common_send(e, &self.tether_agent);
                                }
                            }
                            WidgetEntry::Generic(e) => {
                                if send_if_midi_note(e, &note_message) {
                                    e.publish_from_json_string(&self.tether_agent);
                                }
                            }
                            WidgetEntry::Colour(e) => {
                                if send_if_midi_note(e, &note_message) {
                                    common_send(e, &self.tether_agent);
                                }
                            }
                            WidgetEntry::Point2D(e) => {
                                if send_if_midi_note(e, &note_message) {
                                    common_send(e, &self.tether_agent);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                None => {}
            }
        }
        if !work_done {
            std::thread::sleep(Duration::from_millis(1));
        }

        if self.continuous_mode {
            ctx.request_repaint();
        }

        while let Some(q) = self.queue.pop() {
            match q {
                QueueItem::Remove(index) => {
                    self.project.widgets.remove(index);
                }
            }
        }

        egui::SidePanel::left("General")
            .min_width(256.0)
            .show(ctx, |ui| {
                general_agent_area(ui, self);
            });

        egui::SidePanel::right("Available Widgets")
            .min_width(128.)
            .show(ctx, |ui| {
                ui.heading("Available Widgets");

                standard_spacer(ui);

                available_widgets(ui, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            widgets_in_use(ctx, ui, self);
        });
    }
}
