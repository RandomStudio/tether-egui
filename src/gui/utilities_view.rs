use egui::{Color32, Context, Ui};
use log::*;
use tether_agent::TetherAgentOptionsBuilder;
use tether_utils::{
    tether_playback::{playback, PlaybackOptions},
    tether_topics::MONITOR_LOG_LENGTH,
};

use crate::{tether_gui_utils::init_new_tether_agent, Model};

use super::standard_spacer;

fn insights(ui: &mut Ui, model: &mut Model) {
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

fn message_log(ui: &mut Ui, model: &mut Model) {
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
    ui.small("Simulate timed data, playing from file demo.json");

    let Model {
        editable_tether_settings,
        ..
    } = &model;

    let tether_options = TetherAgentOptionsBuilder::from(editable_tether_settings);

    if ui.button("▶️").clicked() {
        std::thread::spawn(move || {
            match tether_options.auto_connect(true).build() {
                Ok(tether_agent) => {
                    info!("Connected new Tether Agent for playback OK");
                    let options = PlaybackOptions {
                        file_path: "./demo.json".into(),
                        override_topic: None,
                        loop_count: 1,
                        loop_infinite: false,
                    };
                    playback(&options, &tether_agent);
                }
                Err(e) => {
                    error!("Error connecting Tether Agent for playback, {}", e);
                }
            }

            info!("Tether Playback Utility thread completed");
        });
    }
}

pub fn render(ctx: &Context, model: &mut Model) {
    egui::CentralPanel::default().show(ctx, |ui| {
        insights(ui, model);
        standard_spacer(ui);
        ui.separator();
        render_playback(ui, model);
    });

    egui::SidePanel::right("MessageLog")
        .min_width(512.)
        .show(ctx, |ui| {
            message_log(ui, model);
        });
}
