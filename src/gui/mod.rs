use crate::{ActiveView, Model};

use self::{
    common::{general_agent_area, standard_spacer},
    widget_view::{available_widgets, widgets_in_use},
};

pub mod common;
pub mod tether_gui_utils;
pub mod utilities_view;
pub mod widget_view;

fn common_left_panel(ctx: &egui::Context, model: &mut Model) {
    egui::SidePanel::left("General")
        .min_width(256.0)
        .show(ctx, |ui| {
            general_agent_area(ui, model);
        });
}

pub fn render(ctx: &egui::Context, model: &mut Model) {
    egui::TopBottomPanel::top("Tabs").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut model.active_window, ActiveView::WidgetView, "Widgets");
            ui.selectable_value(
                &mut model.active_window,
                ActiveView::UtilitiesView,
                "Utilities",
            );
        })
    });

    match model.active_window {
        ActiveView::WidgetView => {
            common_left_panel(ctx, model);
            egui::SidePanel::right("Available Widgets")
                .min_width(128.)
                .show(ctx, |ui| {
                    ui.heading("Available Widgets");

                    standard_spacer(ui);

                    available_widgets(ui, model);
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                widgets_in_use(ctx, ui, model);
            });
        }
        ActiveView::UtilitiesView => {
            common_left_panel(ctx, model);
            utilities_view::render(ctx, model);
        }
    }
}
