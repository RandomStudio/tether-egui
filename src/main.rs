#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clap::Parser;
use settings::Cli;
use std::fs;
use ui::{available_widgets, general_agent_area, standard_spacer, widget_entries};

extern crate rmp_serde;
extern crate rmpv;
extern crate serde_json;

use eframe::egui;
use env_logger::Env;
use insights::Insights;
use log::{error, info, warn};
use tether_agent::TetherAgent;
use widgets::{Common, WidgetEntry};

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

pub struct Model {
    json_file: Option<String>,
    next_widget: Common,
    next_range: (f32, f32),
    is_valid_json: bool,
    use_custom_topic: bool,
    next_topic: String,
    agent_role: String,
    agent_id: String,
    widgets: Vec<WidgetEntry>,
    queue: Vec<QueueItem>,
    tether_agent: TetherAgent,
    insights: Insights,
    continuous_mode: bool,
    auto_send: bool,
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

        let json_file: String = cli.json_load.unwrap_or(String::from("./widgets.json"));
        info!("Will attempt to load JSON from {} ...", &json_file);
        let load_json = load_widgets_from_disk(&json_file);

        Self {
            json_file: {
                if load_json.is_err() {
                    None
                } else {
                    Some(json_file)
                }
            },
            next_widget,
            next_range: (0., 1.0),
            use_custom_topic: false,
            next_topic,
            agent_role: role.into(),
            agent_id: id.into(),
            widgets: load_json.unwrap_or(Vec::new()),
            queue: Vec::new(),
            insights: Insights::new(&tether_agent, cli.tether_disable),
            tether_agent,
            continuous_mode: cli.continuous_mode,
            is_valid_json: true,
            auto_send: true,
        }
    }
}

fn load_widgets_from_disk(file_path: &str) -> Result<Vec<WidgetEntry>, ()> {
    let text = fs::read_to_string(file_path);
    match text {
        Ok(d) => {
            info!("Found widget data file; parsing...");
            let widgets =
                serde_json::from_str::<Vec<WidgetEntry>>(&d).expect("failed to parse widget list");
            info!("... loaded {} widgets OK", widgets.len());
            // TODO: optionally "broadcast" all values from loaded Widgets
            Ok(widgets)
        }
        Err(e) => {
            error!("Failed to load widgets from disk: {:?}", e);
            Err(())
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

        egui::SidePanel::left("General")
            .min_width(256.0)
            .show(ctx, |ui| {
                general_agent_area(ui, self);
            });

        egui::SidePanel::right("Custom UI")
            .min_width(256.)
            .show(ctx, |ui| {
                ui.heading("Entries");

                standard_spacer(ui);
                // TODO: use grid

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        widget_entries(ui, self);
                    });
            });

        egui::CentralPanel::default().show(ctx, |_ui| {
            available_widgets(ctx, self);
        });
    }
}
