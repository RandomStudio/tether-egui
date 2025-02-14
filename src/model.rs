use std::time::Duration;

use log::{error, info, warn};
use tether_agent::{three_part_topic::TetherOrCustomTopic, TetherAgent, TetherAgentOptionsBuilder};
use tether_utils::tether_topics::{insights::Insights, TopicOptions};

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
    // pub edit_tether_settings: bool,
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

        let tether_agent =
            unconnected_tether_agent(&TetherAgentOptionsBuilder::from(tether_settings));

        let mut init_model = Model {
            tether_agent,
            // edit_tether_settings: false,
            json_file: {
                if was_loaded_from_disk {
                    None
                } else {
                    Some(json_path)
                }
            },
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
        };

        if cli.tether_disable {
            warn!("Tether disabled; please connect manually if required");
        } else {
            init_model.attempt_new_tether_connection();
        }

        init_model
    }
}

impl eframe::App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut work_done = false;
        if let Some(insights) = &mut self.insights {
            insights.sample();
        }
        if self.tether_agent.is_connected() {
            while let Some((topic, payload)) = self.tether_agent.check_messages() {
                work_done = true;
                if let Some(insights) = &mut self.insights {
                    insights.update(&topic, payload.to_vec());
                }
                let plug_name: String = match topic {
                    TetherOrCustomTopic::Custom(topic) => {
                        error!("Invalid Tether Topic \"{}\"", &topic);
                        String::from("INVALID_TETHER_TOPIC!")
                    }
                    TetherOrCustomTopic::Tether(tpt) => String::from(tpt.plug_name()),
                };
                if let Some(midi_handler) = &self.midi_handler {
                    match midi_handler.get_midi_message(&plug_name, &payload) {
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

impl Model {
    /// Always creates a new Tether Agent instance, using the settings either loaded from the
    /// current "project"
    /// or defaults if none are available.
    pub fn attempt_new_tether_connection(&mut self) {
        let tether_settings = match &self.project.tether_settings {
            Some(s) => s.clone(),
            None => EditableTetherSettings::default(),
        };

        self.tether_agent =
            unconnected_tether_agent(&TetherAgentOptionsBuilder::from(tether_settings));

        match self.tether_agent.connect() {
            Ok(()) => {
                info!("Connected Tether Agent OK");
                // model.project.tether_settings.was_changed = true;
                self.insights = Some(Insights::new(
                    &TopicOptions {
                        topic: self.monitor_topic.clone(),
                        sampler_interval: 1000,
                        graph_enable: false,
                    },
                    &mut self.tether_agent,
                ));
                self.midi_handler = Some(MidiSubscriber::new(&mut self.tether_agent));
            }
            Err(e) => {
                error!("Failed to connect Tether Agent: {}", e);
            }
        }
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
