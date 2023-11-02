use std::time::Duration;

use log::{error, info, warn};
use tether_agent::{TetherAgent, TetherAgentOptionsBuilder, TetherOrCustomTopic};
use tether_utils::tether_topics::insights::Insights;

use crate::{
    gui::{
        render,
        tether_gui_utils::{unconnected_tether_agent, EditableTetherSettings},
        utilities_view::{PlaybackState, RecordingState},
        widget_view::common_send,
    },
    midi_mapping::{
        send_if_midi_note, toggle_if_midi_note, update_widget_if_controllable, MidiMessage,
        MidiSubscriber,
    },
    project::{try_load, Project},
    settings::Cli,
    widgets::WidgetEntry,
};
use clap::Parser;

pub struct Model {
    pub tether_agent: TetherAgent,
    pub json_file: Option<String>,
    pub monitor_topic: String,
    pub project: Project,
    pub queue: Vec<QueueItem>,
    pub insights: Option<Insights>,
    pub message_log_filter: String,
    pub midi_handler: Option<MidiSubscriber>,
    pub continuous_mode: bool,
    pub active_window: ActiveView,
    pub playback: PlaybackState,
    pub recording: RecordingState,
}

impl Default for Model {
    fn default() -> Self {
        let cli = Cli::parse();

        let json_path: String = cli.json_load.unwrap_or(String::from("./project.json"));
        info!("Will attempt to load JSON from {} ...", &json_path);

        let (project, was_loaded_from_disk) = try_load(&json_path);

        let tether_settings = match &project.tether_settings {
            Some(s) => s.clone(),
            None => EditableTetherSettings::default(),
        };

        // let tether_settings = project.tether_settings

        // let tether_agent = init_new_tether_agent(&TetherAgentOptionsBuilder::from(
        //     project.tether_settings.unwrap_or_default().clone(),
        // ));

        // if cli.tether_disable {
        //     warn!("Tether disabled; please connect manually if required");
        // } else {
        //     match tether_agent.connect() {
        //         Ok(()) => {
        //             info!("Tether Agent connected successfully");
        //         }
        //         Err(e) => {
        //             error!("Tether Agent failed to connect: {}", e);
        //         }
        //     }
        // }

        let tether_agent =
            unconnected_tether_agent(&TetherAgentOptionsBuilder::from(tether_settings));

        Self {
            json_file: {
                if was_loaded_from_disk {
                    None
                } else {
                    Some(json_path)
                }
            },
            tether_agent,
            monitor_topic: cli.monitor_topic.clone(),
            project,
            queue: Vec::new(),
            insights: None,
            message_log_filter: "".into(),
            midi_handler: None,
            continuous_mode: cli.continuous_mode,
            active_window: ActiveView::WidgetView,
            playback: PlaybackState::default(),
            recording: RecordingState::default(),
        }
    }
}

impl eframe::App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut work_done = false;
        if let Some(insights) = &mut self.insights {
            insights.sample();
        }
        if self.tether_agent.is_connected() {
            while let Some((plug, message)) = &self.tether_agent.check_messages() {
                work_done = true;
                if let Some(insights) = &mut self.insights {
                    insights.update(message);
                }
                let plug_name = match plug {
                    TetherOrCustomTopic::Custom(topic) => {
                        error!("Invalid Tether Topic \"{}\"", &topic);
                        "INVALID_TETHER_TOPIC!"
                    }
                    TetherOrCustomTopic::Tether(tpt) => tpt.plug_name(),
                };
                if let Some(midi_handler) = &self.midi_handler {
                    match midi_handler.get_midi_message(plug_name, message) {
                        Some(MidiMessage::ControlChange(cc_message)) => {
                            for widget in self.project.widgets.iter_mut() {
                                match widget {
                                    WidgetEntry::FloatNumber(e) => {
                                        update_widget_if_controllable(
                                            e,
                                            &cc_message,
                                            &self.tether_agent,
                                        );
                                    }
                                    WidgetEntry::WholeNumber(e) => {
                                        update_widget_if_controllable(
                                            e,
                                            &cc_message,
                                            &self.tether_agent,
                                        );
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

        render(ctx, self);
    }
}

#[derive(PartialEq)]
pub enum ActiveView {
    WidgetView,
    UtilitiesView,
}

pub enum QueueItem {
    Remove(usize),
}
