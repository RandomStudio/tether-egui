use std::{sync::mpsc, thread::JoinHandle};

use egui::{
    plot::{Line, Plot, PlotPoints},
    Color32, Context, RichText, Ui,
};
use log::*;
use tether_agent::TetherAgentOptionsBuilder;
use tether_utils::{
    tether_playback::{PlaybackOptions, TetherPlaybackUtil},
    tether_record::{RecordOptions, TetherRecordUtil},
    tether_topics::insights::MONITOR_LOG_LENGTH,
};

use crate::Model;

use super::{common::standard_spacer, tether_gui_utils::EditableTetherSettings};

#[derive(Default)]
pub struct PlaybackState {
    options: Option<PlaybackOptions>,
    // file_path: Option<String>,
    is_playing: bool,
    thread_handle: Option<JoinHandle<()>>,
    stop_request_tx: Option<mpsc::Sender<bool>>,
    // loop_infinite: bool
}

pub struct RecordingState {
    options: RecordOptions,
    is_recording: bool,
    thread_handle: Option<JoinHandle<()>>,
    stop_request_tx: Option<mpsc::Sender<bool>>,
}

impl Default for RecordingState {
    fn default() -> Self {
        RecordingState {
            options: RecordOptions {
                ignore_ctrl_c: true,
                ..RecordOptions::default()
            },
            is_recording: false,
            thread_handle: None,
            stop_request_tx: None,
        }
    }
}

fn render_insights(ui: &mut Ui, model: &mut Model) {
    ui.heading("Insights");
    ui.checkbox(&mut model.continuous_mode, "Continuous mode")
        .on_hover_text("Message log will update immediately; CPU usage may be higher");

    standard_spacer(ui);

    if let Some(insights) = &mut model.insights {
        ui.columns(2, |columns| {
            // Column left
            let ui = &mut columns[0];

            ui.heading("List view");

            ui.label(format!("Topics x{}", insights.topics().len()));
            for t in insights.topics() {
                ui.small(format!(" - {}", t));
            }

            ui.label(format!("Agent Roles x{}", insights.roles().len()));
            for role in insights.roles() {
                ui.small(format!(" - {}", role));
            }

            ui.label(format!("Agent IDs x{}", insights.ids().len()));
            for id in insights.roles() {
                ui.small(format!(" - {}", id));
            }

            ui.label(format!("Plug Names x{}", insights.plugs().len()));
            for plug in insights.plugs() {
                ui.small(format!(" - {}", plug));
            }

            // Column right
            let ui = &mut columns[1];
            ui.heading("Tree view");

            insights.trees().iter().for_each(|agent_tree| {
                ui.group(|ui| {
                    ui.heading(&agent_tree.role);
                    agent_tree.ids.iter().for_each(|id| {
                        let formatted = if id.len() > 12 {
                            let mut shorter = id.clone();
                            shorter.truncate(12);
                            format!("{}...", shorter)
                        } else {
                            id.into()
                        };
                        ui.label(format!("🏠 {}", formatted)).on_hover_text(id);
                    });
                    agent_tree.output_plugs.iter().for_each(|plug| {
                        ui.label(format!(" ----🔌 {}", plug));
                    });
                });
            });
        });
        ui.separator();

        ui.heading("Throughput Graph");
        ui.label(format!(
            "Averaging {:.2} messages per second",
            insights.get_rate().unwrap_or(0.)
        ));
        let line = Line::new(PlotPoints::from_iter(
            insights
                .sampler()
                .delta_entries()
                .iter()
                .enumerate()
                .map(|(i, x)| [i as f64, *x as f64]),
        ));
        let plot = Plot::new("messages");
        plot.show(ui, |plot_ui| {
            plot_ui.line(line);
        });
    };
}

fn render_message_log(ui: &mut Ui, model: &mut Model) {
    ui.group(|ui| {
        ui.label("Topic filter");
        ui.text_edit_singleline(&mut model.message_log_filter);
    });

    if let Some(insights) = &mut model.insights {
        ui.heading(format!("Messages x{}", insights.message_count()));
        if insights.message_log().is_empty() {
            ui.small("0 messages in log");
        } else {
            ui.small(format!(
                "showing {} messages in log (up to {})",
                insights.message_log().len(),
                MONITOR_LOG_LENGTH
            ));
        }

        standard_spacer(ui);

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (topic, json) in insights
                    .message_log()
                    .iter()
                    .filter(|(topic, _json)| {
                        if model.message_log_filter.is_empty() {
                            true
                        } else {
                            topic.contains(&model.message_log_filter)
                        }
                    })
                    .rev()
                {
                    ui.colored_label(Color32::LIGHT_BLUE, topic);
                    ui.label(json);
                }
            });
    }
}

fn render_playback(ui: &mut Ui, model: &mut Model) {
    ui.label("Simulate timed messages");

    if ui.button("Load").clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("text", &["json"])
            .pick_file()
        {
            let file_path = path.display().to_string();
            model.playback.options = Some(PlaybackOptions {
                file_path,
                ignore_ctrl_c: true,
                loop_infinite: true,
                ..PlaybackOptions::default()
            });
        }
    }

    match &mut model.playback.options {
        Some(options) => {
            ui.label(format!("Play from \"{}\"", options.file_path));

            ui.horizontal(|ui| {
                ui.checkbox(&mut options.loop_infinite, "Loop infinite");
                if !options.loop_infinite {
                    ui.label("Iterations:");
                    ui.add(egui::DragValue::new(&mut options.loop_count).speed(1.0));
                }
            });

            ui.horizontal(|ui| {
                if !model.playback.is_playing {
                    if ui.button("⏵ Play").clicked() {
                        model.playback.is_playing = true;
                        let player = TetherPlaybackUtil::new(options.to_owned());

                        let tether_settings = match &model.project.tether_settings {
                            Some(s) => s.clone(),
                            None => EditableTetherSettings::default(),
                        };

                        model.playback.stop_request_tx = Some(player.get_stop_tx());
                        model.playback.thread_handle = Some(std::thread::spawn(move || {
                            if let Ok(mut tether_agent) =
                                TetherAgentOptionsBuilder::from(tether_settings).build()
                            {
                                tether_agent.connect().expect("failed to connect");
                                info!("Connected new Tether Agent for playback OK");
                                player.start(&tether_agent);
                            } else {
                                error!("Failed tonnect Tether Agent for playback");
                            }
                        }));
                    }
                } else if ui.button("⏹ Stop").clicked() {
                    if let Some(tx) = &model.playback.stop_request_tx {
                        tx.send(true)
                            .expect("failed to send playback stop request via channel");
                    } else {
                        panic!(
                            "Playback was marked in-progress but no stop request channel available"
                        );
                    }
                }
            });
            if model.playback.is_playing {
                if let Some(handle) = &model.playback.thread_handle {
                    if handle.is_finished() {
                        info!("Playback thread finished");
                        model.playback.is_playing = false;
                        model.playback.thread_handle = None;
                        model.playback.stop_request_tx = None;
                    }
                }
            }
        }
        None => {
            ui.label("No playback file loaded");
        }
    }
}

fn render_record(ui: &mut Ui, model: &mut Model) {
    ui.label("Record message, with timing, for simulation");

    egui::Grid::new("my_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("File basename")
                .on_hover_text("Excluding .json extension");
            ui.text_edit_singleline(&mut model.recording.options.file_base_name);
            ui.end_row();

            ui.label("Path");
            ui.text_edit_singleline(&mut model.recording.options.file_base_path);
            ui.end_row();

            ui.label("Timestamp");
            ui.horizontal(|ui| {
                if model.recording.options.file_no_timestamp {
                    ui.label(RichText::new("Disabled").color(Color32::DARK_GRAY));
                    if ui.button("Enable").clicked() {
                        model.recording.options.file_no_timestamp = false;
                    }
                } else {
                    ui.label("Enabled");
                    if ui.button("Disable").clicked() {
                        model.recording.options.file_no_timestamp = true;
                    }
                }
            });
            ui.end_row();

            ui.label("Timing Delay");
            ui.horizontal(|ui| {
                if let Some(delay) = &mut model.recording.options.timing_delay {
                    ui.add(egui::DragValue::new(delay).speed(1.0));
                    if ui.button("disable").clicked() {
                        model.recording.options.timing_delay = None;
                    }
                } else {
                    ui.label(RichText::new("Disabled").color(Color32::DARK_GRAY));
                    if ui.button("Enable").clicked() {
                        model.recording.options.timing_delay = Some(2.0);
                    }
                }
            });
            ui.end_row();

            ui.label("Max Duration");
            ui.horizontal(|ui| {
                if let Some(duration) = &mut model.recording.options.timing_duration {
                    ui.add(egui::DragValue::new(duration).speed(1.0));
                    if ui.button("Disable").clicked() {
                        model.recording.options.timing_duration = None;
                    }
                } else {
                    ui.label(RichText::new("Disabled").color(Color32::DARK_GRAY));
                    if ui.button("Enable").clicked() {
                        model.recording.options.timing_duration = Some(10.0);
                    }
                }
            });
        });
    standard_spacer(ui);
    ui.separator();
    ui.horizontal(|ui| {
        if !model.recording.is_recording {
            if ui.button("⏺ Record").clicked() {
                model.recording.is_recording = true;
                let recorder = TetherRecordUtil::new(model.recording.options.to_owned());

                let tether_settings = match &model.project.tether_settings {
                    Some(s) => s.clone(),
                    None => EditableTetherSettings::default(),
                };

                if let Ok(mut tether_agent) =
                    TetherAgentOptionsBuilder::from(tether_settings).build()
                {
                    model.recording.stop_request_tx = Some(recorder.get_stop_tx());
                    model.recording.thread_handle = Some(std::thread::spawn(move || {
                        tether_agent.connect().expect("failed to connect");
                        info!("Connected new Tether Agent for recording OK");
                        recorder.start_recording(&mut tether_agent);
                    }));
                } else {
                    error!("Failed to connect Tether Agent for recording");
                }
            }
        } else if ui.button("⏹ Stop").clicked() {
            if let Some(tx) = &model.recording.stop_request_tx {
                tx.send(true)
                    .expect("failed to send recording stop request via channel");
            } else {
                panic!("Recording was marked in-progress but no stop request channel available");
            }
        }
    });
    if model.recording.is_recording {
        if let Some(handle) = &model.recording.thread_handle {
            if handle.is_finished() {
                info!("Recording thread finished");
                model.recording.is_recording = false;
                model.recording.thread_handle = None;
                model.recording.stop_request_tx = None;
            }
        }
    }
}

pub fn render(ctx: &Context, model: &mut Model) {
    if model.insights.is_some() {
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::Window::new("Insights").show(ctx, |ui| {
                render_insights(ui, model);
            });
            egui::Window::new("Playback")
                .default_pos([0., ctx.used_rect().height() * 0.5])
                .show(ctx, |ui| {
                    render_playback(ui, model);
                });
            egui::Window::new("Recording")
                .default_pos([0., ctx.used_rect().height() * 0.7])
                .show(ctx, |ui| {
                    render_record(ui, model);
                });
        });

        egui::SidePanel::right("MessageLog")
            .min_width(512.)
            .show(ctx, |ui| {
                render_message_log(ui, model);
            });
    } else {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(
                RichText::new("Utilities will become available on successful connection.")
                    .color(Color32::YELLOW),
            );
        });
    }
}
