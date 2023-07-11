use std::{sync::mpsc, thread::JoinHandle};

use egui::{Color32, Context, Ui};
use log::*;
use tether_agent::TetherAgentOptionsBuilder;
use tether_utils::{
    tether_playback::{PlaybackOptions, TetherPlaybackUtil},
    tether_topics::MONITOR_LOG_LENGTH,
};

use crate::Model;

use super::standard_spacer;

pub struct PlaybackState {
    options: Option<PlaybackOptions>,
    // file_path: Option<String>,
    is_playing: bool,
    thread_handle: Option<JoinHandle<()>>,
    stop_request_tx: Option<mpsc::Sender<bool>>,
    // loop_infinite: bool

}

impl Default for PlaybackState {
    fn default() -> Self {
        // let options = PlaybackOptions {   ignore_ctrl_c: true, ..PlaybackOptions::default() };
        PlaybackState {
            options: None,
            is_playing: false,
            stop_request_tx: None,
            thread_handle: None,
        }
    }
}

fn render_insights(ui: &mut Ui, model: &mut Model) {
    ui.heading("Insights");
    ui.checkbox(&mut model.continuous_mode, "Continuous mode")
        .on_hover_text("Message log will update immediately; CPU usage may be higher");

    standard_spacer(ui);

    ui.heading(format!("Topics x{}", model.insights.topics().len()));
    for t in model.insights.topics() {
        ui.small(t);
    }

    ui.heading(format!("Agent Roles x{}", model.insights.roles().len()));
    for role in model.insights.roles() {
        ui.label(role);
    }

    ui.heading(format!("Agent IDs x{}", model.insights.ids().len()));
    for id in model.insights.roles() {
        ui.label(id);
    }

    ui.heading(format!("Plug Names x{}", model.insights.plugs().len()));
    for plug in model.insights.plugs() {
        ui.label(plug);
    }
}

fn render_message_log(ui: &mut Ui, model: &mut Model) {
    ui.heading(format!("Messages x{}", model.insights.message_count()));
    if model.insights.message_log().is_empty() {
        ui.small("0 messages in log");
    } else {
        ui.small(format!(
            "showing {} messages in log (up to {})",
            model.insights.message_log().len(),
            MONITOR_LOG_LENGTH
        ));
    }

    standard_spacer(ui);

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for (topic, json) in model.insights.message_log().iter().rev() {
                ui.colored_label(Color32::LIGHT_BLUE, topic);
                ui.label(json);
            }
        });
}

fn render_playback(ui: &mut Ui, model: &mut Model) {
    ui.heading("Playback");
    ui.label("Simulate timed data");

    if ui.button("Load").clicked() {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("text", &["json"])
            .pick_file()
        {
            let file_path = path.display().to_string();
            model.playback.options = Some(PlaybackOptions { file_path,  ignore_ctrl_c: true, loop_infinite: true, ..PlaybackOptions::default() } );
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
                
                if !model.playback.is_playing  {
                    if ui.button("⏵ Play").clicked() {
                        model.playback.is_playing = true;
                        let player = TetherPlaybackUtil::new(options.to_owned());
                        model.playback.stop_request_tx = Some(player.get_stop_tx());
        
                        model.playback.thread_handle = Some(std::thread::spawn(move || {
                            let tether_agent = TetherAgentOptionsBuilder::new("playbackAgent")
                                .build()
                                .expect("failed to init/connect Tether for playback");
                            info!("Connected new Tether Agent for playback OK");
                            player.start(&tether_agent);
                        }));
                    }

                }else {
                    if ui.button("⏹ Stop").clicked() {
                        if let Some(tx) = &model.playback.stop_request_tx {
                            tx.send(true)
                                .expect("failed to send stop request via channel");
                        } else {
                            panic!(
                                "Playback was marked in-progress but no stop request channel available"
                            );
                        }
                    }
                }
            });
            if model.playback.is_playing {
                if let Some(handle) = &model.playback.thread_handle {
                    if handle.is_finished() {
                        info!("Playback handle finished");
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

pub fn render(ctx: &Context, model: &mut Model) {
    egui::CentralPanel::default().show(ctx, |ui| {
        render_insights(ui, model);
        standard_spacer(ui);
        ui.separator();
        render_playback(ui, model);
    });

    egui::SidePanel::right("MessageLog")
        .min_width(512.)
        .show(ctx, |ui| {
            render_message_log(ui, model);
        });
}
