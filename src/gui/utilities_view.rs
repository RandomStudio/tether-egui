use egui::{Color32, Context, Ui};
use tether_utils::tether_topics::MONITOR_LOG_LENGTH;

use crate::Model;

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

pub fn render(ctx: &Context, model: &mut Model) {
    egui::CentralPanel::default().show(ctx, |ui| {
        insights(ui, model);
    });

    egui::SidePanel::right("MessageLog")
        .min_width(512.)
        .show(ctx, |ui| {
            message_log(ui, model);
        });
}
