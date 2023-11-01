use egui::{Color32, Response, RichText, Ui};
use log::{debug, error};
use serde::Serialize;
use tether_agent::{PlugOptionsBuilder, TetherAgent};

use crate::{
    midi_mapping::MidiMapping,
    widgets::{
        boolean::BoolWidget, colours::ColourWidget, empty::EmptyWidget, generic::GenericJSONWidget,
        numbers::NumberWidget, point::Point2DWidget, shortened_name, CustomWidget, Qos, View,
        WidgetEntry,
    },
    Model, QueueItem,
};

use super::common::{common_remove_button, standard_spacer};

pub fn common_in_use_heading<T: Serialize>(ui: &mut egui::Ui, entry: &mut impl CustomWidget<T>) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(&entry.common().name)
                .color(Color32::WHITE)
                .size(18.),
        );
        if ui.button("edit").clicked() {
            entry.common_mut().set_edit_mode(true);
        }
    });
    ui.small(&entry.common().description);
    ui.separator();
}

pub fn common_save_button<T: Serialize>(
    ui: &mut egui::Ui,
    entry: &mut impl CustomWidget<T>,
    tether_agent: &TetherAgent,
) {
    if ui.button("Save").clicked() {
        update_plug_definition(entry, tether_agent);
        entry.common_mut().set_edit_mode(false);
    }
}

pub fn common_send_button<T: Serialize>(
    ui: &mut egui::Ui,
    entry: &mut impl CustomWidget<T>,
    allow_auto_send: bool,
) -> Response {
    let res = ui.horizontal(|ui| {
        let res = ui.button("Send");
        entry_topic(ui, entry);
        if allow_auto_send {
            ui.checkbox(&mut entry.common_mut().auto_send, "Auto send");
        }
        res
    });
    res.inner
}

pub fn common_send<T: Serialize>(entry: &mut impl CustomWidget<T>, tether_agent: &TetherAgent) {
    match tether_agent.encode_and_publish(&entry.common().plug, entry.value()) {
        Ok(()) => debug!("Send OK"),
        Err(_) => error!(
            "Failed to send via Tether; connected? {}",
            tether_agent.is_connected()
        ),
    }
}

pub fn entry_topic<T: Serialize>(ui: &mut egui::Ui, entry: &impl CustomWidget<T>) {
    ui.label(
        RichText::new(format!("Topic: {}", entry.common().plug.topic())).color(Color32::LIGHT_BLUE),
    );
}

pub fn widgets_in_use(ctx: &egui::Context, ui: &mut Ui, model: &mut Model) {
    let widgets = &mut model.project.widgets;

    for (i, entry) in widgets.iter_mut().enumerate() {
        match entry {
            WidgetEntry::FloatNumber(e) => {
                egui::Window::new(&e.common().name)
                    .id(format!("{}", i).into())
                    .show(ctx, |ui| {
                        if e.common().is_edit_mode() {
                            e.render_editing(ui, &model.tether_agent);
                            if common_remove_button(ui) {
                                model.queue.push(QueueItem::Remove(i));
                            }
                        } else {
                            e.render_in_use(ui, &model.tether_agent);
                        }
                    });
            }
            WidgetEntry::WholeNumber(e) => {
                egui::Window::new(&e.common().name)
                    .id(format!("{}", i).into())
                    .show(ctx, |ui| {
                        if e.common().is_edit_mode() {
                            e.render_editing(ui, &model.tether_agent);
                            if common_remove_button(ui) {
                                model.queue.push(QueueItem::Remove(i));
                            }
                        } else {
                            e.render_in_use(ui, &model.tether_agent);
                        }
                    });
            }
            WidgetEntry::Colour(e) => {
                egui::Window::new(&e.common().name)
                    .id(format!("{}", i).into())
                    .show(ctx, |ui| {
                        if e.common().is_edit_mode() {
                            e.render_editing(ui, &model.tether_agent);
                            if common_remove_button(ui) {
                                model.queue.push(QueueItem::Remove(i));
                            }
                        } else {
                            e.render_in_use(ui, &model.tether_agent);
                        }
                    });
            }
            WidgetEntry::Bool(e) => {
                egui::Window::new(&e.common().name)
                    .id(format!("{}", i).into())
                    .show(ctx, |ui| {
                        if e.common().is_edit_mode() {
                            e.render_editing(ui, &model.tether_agent);
                            if common_remove_button(ui) {
                                model.queue.push(QueueItem::Remove(i));
                            }
                        } else {
                            e.render_in_use(ui, &model.tether_agent);
                        }
                    });
            }
            WidgetEntry::Empty(e) => {
                egui::Window::new(&e.common().name)
                    .id(format!("{}", i).into())
                    .show(ctx, |ui| {
                        if e.common().is_edit_mode() {
                            e.render_editing(ui, &model.tether_agent);
                            if common_remove_button(ui) {
                                model.queue.push(QueueItem::Remove(i));
                            }
                        } else {
                            e.render_in_use(ui, &model.tether_agent);
                        }
                    });
            }
            WidgetEntry::Point2D(e) => {
                egui::Window::new(&e.common().name)
                    .id(format!("{}", i).into())
                    .show(ctx, |ui| {
                        if e.common().is_edit_mode() {
                            e.render_editing(ui, &model.tether_agent);
                            if common_remove_button(ui) {
                                model.queue.push(QueueItem::Remove(i));
                            }
                        } else {
                            e.render_in_use(ui, &model.tether_agent);
                        }
                    });
            }
            WidgetEntry::Generic(e) => {
                egui::Window::new(&e.common().name)
                    .id(format!("{}", i).into())
                    .show(ctx, |ui| {
                        if e.common().is_edit_mode() {
                            e.render_editing(ui, &model.tether_agent);
                            if common_remove_button(ui) {
                                model.queue.push(QueueItem::Remove(i));
                            }
                        } else {
                            e.render_in_use(ui, &model.tether_agent);
                        }
                    });
            }
        }

        ui.end_row();

        standard_spacer(ui);

        ui.end_row();
    }
}

pub fn available_widgets(ui: &mut egui::Ui, model: &mut Model) {
    if ui.button("Boolean").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Bool(BoolWidget::new(
                "Boolean Meassage",
                Some("A true or false value"),
                "booleans",
                None,
                false,
                &model.tether_agent,
            )));
    }
    if ui.button("Empty").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Empty(EmptyWidget::new(
                "Empty Meassage",
                Some("A message with no payload"),
                "events",
                None,
                &model.tether_agent,
            )));
    }

    if ui.button("Floating Point").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::FloatNumber(NumberWidget::new(
                "Floating Point Number",
                Some("A single 64-bit floating point number"),
                "floats",
                None,
                0.,
                0. ..=1.0,
                false,
                &model.tether_agent,
            )));
    }
    if ui.button("Whole Number").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::WholeNumber(NumberWidget::new(
                "Whole Number",
                Some("A single 64-bit whole number"),
                "numbers",
                None,
                0.,
                0. ..=100.,
                true,
                &model.tether_agent,
            )));
    }
    if ui.button("Point2D").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Point2D(Point2DWidget::new(
                "Point2D",
                Some("X and Y values"),
                "point2d",
                None,
                &model.tether_agent,
            )));
    }
    if ui.button("Generic data").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Generic(GenericJSONWidget::new(
                "Generic JSON Data",
                Some("Any generic data, in JSON format"),
                "generic",
                None,
                &model.tether_agent,
            )));
    }
    if ui.button("Colour").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Colour(ColourWidget::new(
                "Colour",
                Some("8-bit colour including alpha"),
                "colours",
                None,
                &model.tether_agent,
            )))
    }
}

pub fn common_editable_values<T: Serialize>(
    ui: &mut egui::Ui,
    entry: &mut impl CustomWidget<T>,
    tether_agent: &TetherAgent,
) {
    ui.label("Widget Name");
    if ui
        .text_edit_singleline(&mut entry.common_mut().name)
        .changed()
    {
        update_plug_definition(entry, tether_agent);
    }

    ui.label("Description");
    ui.text_edit_multiline(&mut entry.common_mut().description);

    ui.label("Plug Name");
    if ui
        .text_edit_singleline(&mut entry.common_mut().plug_name)
        .changed()
    {
        // Back to default (auto-generated) plug name, details
        update_plug_definition(entry, tether_agent);
    }

    if ui
        .checkbox(&mut entry.common_mut().use_custom_topic, "Use custom topic")
        .changed()
        && !entry.common().use_custom_topic
    {
        // Back to default (auto-generated) topic
        update_plug_definition(entry, tether_agent);
    }
    ui.add_enabled_ui(entry.common().use_custom_topic, |ui| {
        if ui
            .text_edit_singleline(&mut entry.common_mut().custom_topic)
            .changed()
        {
            update_plug_definition(entry, tether_agent);
        }
    });

    common_edit_midi_mapping(ui, entry);

    ui.collapsing("Publish options", |ui| {
        ui.group(|ui| {
            ui.label("QOS level");
            ui.radio_value(
                &mut entry.common_mut().qos,
                Qos::AtLeastOnce,
                "0: At least once",
            )
            .on_hover_text("Fastest, no delivery guarrantees");
            ui.radio_value(
                &mut entry.common_mut().qos,
                Qos::AtMostOnce,
                "1: At most once",
            )
            .on_hover_text("Ensure delivery, duplicates possible");
            ui.radio_value(
                &mut entry.common_mut().qos,
                Qos::ExactlyOnce,
                "2: Exactly once",
            )
            .on_hover_text("Slowest, guarranteed once-only delivery");
        });
        ui.group(|ui| {
            ui.checkbox(&mut entry.common_mut().retain, "Retain?");
        });
    });
}

fn update_plug_definition<T: Serialize>(
    entry: &mut impl CustomWidget<T>,
    tether_agent: &TetherAgent,
) {
    debug!("Will update plug definition");
    debug!("QOS level: {}", entry.common().qos as i32);
    debug!("Retain: {}", entry.common().retain);

    entry.common_mut().plug = PlugOptionsBuilder::create_output(&entry.common().plug_name)
        .qos(Some(entry.common().qos as i32))
        .retain(Some(entry.common().retain))
        .topic(if entry.common().use_custom_topic {
            Some(&entry.common().custom_topic)
        } else {
            None
        })
        .build(tether_agent)
        .expect("failed to create output")
}

pub fn common_edit_midi_mapping<T: Serialize>(ui: &mut egui::Ui, entry: &mut impl CustomWidget<T>) {
    if ui.button("Learn MIDI mapping").clicked() {
        entry.common_mut().midi_mapping = Some(MidiMapping::Learning);
    }
    if let Some(midi) = &entry.common().midi_mapping {
        match midi {
            MidiMapping::Learning => {
                ui.label("Learning...");
            }
            MidiMapping::Set(mapping) => {
                ui.label(format!("{:?}", mapping));
            }
        }
    }
}
