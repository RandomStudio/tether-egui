#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clap::Parser;
use midi_mapping::MidiSubscriber;
use project::EditableTetherSettings;
use settings::Cli;
use ui::{available_widgets, general_agent_area, standard_spacer, widgets_in_use};

extern crate rmp_serde;
extern crate rmpv;
extern crate serde_json;

use eframe::egui;
use env_logger::Env;
use insights::Insights;
use log::{debug, error, info, warn};
use tether_agent::TetherAgent;
use widgets::WidgetEntry;

use crate::{
    midi_mapping::update_widget_if_controllable,
    project::{Project, TetherSettings},
    settings::LOCALHOST,
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

pub struct Model {
    json_file: Option<String>,
    agent_role: String,
    agent_id: String,
    monitor_topic: String,
    project: Project,
    queue: Vec<QueueItem>,
    tether_agent: TetherAgent,
    insights: Insights,
    midi_handler: MidiSubscriber,
    continuous_mode: bool,
    editable_tether_settings: EditableTetherSettings,
}

impl Default for Model {
    fn default() -> Self {
        let cli = Cli::parse();

        let mut project = Project::default();

        let json_path: String = cli.json_load.unwrap_or(String::from("./project.json"));
        info!("Will attempt to load JSON from {} ...", &json_path);

        let project_loaded = project.load(&json_path);

        let tether_settings = project.tether_settings.clone().unwrap_or(TetherSettings {
            host: cli.tether_host.to_string(),
            username: cli.tether_username,
            password: cli.tether_password,
        });

        let tether_agent = TetherAgent::new(
            "gui",
            None,
            Some(tether_settings.host.parse().unwrap_or(LOCALHOST)),
        );
        let (role, id) = tether_agent.description();

        if cli.tether_disable {
            warn!("Tether disabled; please connect manually if required");
        } else {
            match tether_agent.connect(tether_settings.username, tether_settings.password) {
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
            agent_role: role.into(),
            agent_id: id.into(),
            monitor_topic: cli.monitor_topic.clone(),
            project,
            queue: Vec::new(),
            insights: Insights::new(&tether_agent, &cli.monitor_topic),
            midi_handler: MidiSubscriber::new(&tether_agent),
            tether_agent,
            continuous_mode: cli.continuous_mode,
            editable_tether_settings: EditableTetherSettings::default(),
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
        while let Some((plug_name, message)) = &self.tether_agent.check_messages() {
            self.insights.update(plug_name, message);
            if let Some(control_change_message) =
                self.midi_handler.get_controller_message(plug_name, message)
            {
                debug!("Got ControlChange message: {:?}", &control_change_message);
                for widget in self.project.widgets.iter_mut() {
                    match widget {
                        WidgetEntry::FloatNumber(e) => {
                            if update_widget_if_controllable(e, &control_change_message) {
                                common_send(e, &self.tether_agent);
                            }
                        }
                        _ => {}
                    }
                }
            }
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
