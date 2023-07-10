use egui::{Color32, Context, Ui};

use crate::Model;

use super::standard_spacer;

fn insights(ui: &mut Ui, model: &mut Model) {
    ui.heading("Insights");
    ui.checkbox(&mut model.continuous_mode, "Continuous mode")
        .on_hover_text("Message log will update immediately; CPU usage may be higher");
    ui.collapsing(format!("Topics x{}", model.insights.topics().len()), |ui| {
        for t in model.insights.topics() {
            ui.small(t);
        }
    });
    ui.collapsing(
        format!("Plug Names x{}", model.insights.plugs().len()),
        |ui| {
            for t in model.insights.plugs() {
                ui.label(t);
            }
        },
    );
    ui.collapsing(
        format!("Agent Roles x{}", model.insights.roles().len()),
        |ui| {
            for t in model.insights.roles() {
                ui.label(t);
            }
        },
    );
    ui.collapsing(
        format!("Agent IDs (groups) x{}", model.insights.ids().len()),
        |ui| {
            for t in model.insights.ids() {
                ui.label(t);
            }
        },
    );
}

fn message_log(ui: &mut Ui, model: &mut Model) {
    ui.heading(format!("Messages x{}", model.insights.message_count()));
    if model.insights.message_log().is_empty() {
        ui.small("0 messages received");
    }

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
