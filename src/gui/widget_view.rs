use egui::{Color32, Response, RichText, Ui};
use log::{debug, error};
use serde::Serialize;
use tether_agent::{
    ChannelDef, ChannelDefBuilder, ChannelSenderDefBuilder, TetherAgent, mqtt::QoS,
};

use crate::{
    Model,
    midi_mapping::MidiMapping,
    model::QueueItem,
    widgets::{
        CustomWidget, View, WidgetEntry, boolean::BoolWidget, colours::ColourWidget,
        empty::EmptyWidget, generic::GenericJSONWidget, numbers::NumberWidget,
        point::Point2DWidget,
    },
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
    tether_agent: &mut TetherAgent,
) {
    if ui.button("Save").clicked() {
        update_channel_definition(entry, tether_agent);
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
    let payload = rmp_serde::to_vec_named(&entry.value()).expect("common_send failed to encode");
    match tether_agent.send_raw(&entry.common().channel_def, Some(&payload)) {
        Ok(()) => debug!("Send OK"),
        Err(_) => error!(
            "Failed to send via Tether; connected? {}",
            tether_agent.is_connected()
        ),
    }
}

pub fn entry_topic<T: Serialize>(ui: &mut egui::Ui, entry: &impl CustomWidget<T>) {
    ui.label(
        RichText::new(format!(
            "Topic: {}",
            entry.common().channel_def.generated_topic()
        ))
        .color(Color32::LIGHT_BLUE),
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
                            e.render_editing(ui, &mut model.tether_agent);
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
                            e.render_editing(ui, &mut model.tether_agent);
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
                            e.render_editing(ui, &mut model.tether_agent);
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
                            e.render_editing(ui, &mut model.tether_agent);
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
                            e.render_editing(ui, &mut model.tether_agent);
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
                            e.render_editing(ui, &mut model.tether_agent);
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
                            e.render_editing(ui, &mut model.tether_agent);
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
                "Boolean Messages",
                Some("A true or false value"),
                None,
                None,
                false,
                &mut model.tether_agent,
            )));
    }
    if ui.button("Empty").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Empty(EmptyWidget::new(
                "Empty Meassages",
                Some("A message with no payload"),
                None,
                None,
                &mut model.tether_agent,
            )));
    }

    if ui.button("Floating Point").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::FloatNumber(NumberWidget::new(
                "Floating Point Numbers",
                Some("A single 64-bit floating point number"),
                None,
                None,
                0.,
                0. ..=1.0,
                false,
                &mut model.tether_agent,
            )));
    }
    if ui.button("Whole Number").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::WholeNumber(NumberWidget::new(
                "Whole Numbers",
                Some("A single 64-bit whole number"),
                None,
                None,
                0.,
                0. ..=100.,
                true,
                &mut model.tether_agent,
            )));
    }
    if ui.button("Point2D").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Point2D(Point2DWidget::new(
                "2DPoints",
                Some("X and Y values"),
                None,
                None,
                &mut model.tether_agent,
            )));
    }
    if ui.button("Generic data").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Generic(GenericJSONWidget::new(
                "Generic Messages",
                Some("Any generic data, in JSON format"),
                None,
                None,
                &mut model.tether_agent,
            )));
    }
    if ui.button("Colour").clicked() {
        model
            .project
            .widgets
            .push(WidgetEntry::Colour(ColourWidget::new(
                "Colours",
                Some("8-bit colour including alpha"),
                None,
                None,
                &mut model.tether_agent,
            )))
    }
}

pub fn common_editable_values<T: Serialize>(
    ui: &mut egui::Ui,
    entry: &mut impl CustomWidget<T>,
    tether_agent: &mut TetherAgent,
) {
    ui.label("Widget Name");
    if ui
        .text_edit_singleline(&mut entry.common_mut().name)
        .changed()
    {
        update_channel_definition(entry, tether_agent);
    }

    ui.label("Description");
    ui.text_edit_multiline(&mut entry.common_mut().description);

    ui.label("Channel Name");
    if ui
        .text_edit_singleline(&mut entry.common_mut().channel_name)
        .changed()
    {
        // Back to default (auto-generated) channel name, details
        update_channel_definition(entry, tether_agent);
    }

    let mut was_custom_topic_enabled = entry.common().custom_topic.is_some();

    if ui
        .checkbox(&mut was_custom_topic_enabled, "Use custom topic")
        .changed()
    {
        let just_enabled = !was_custom_topic_enabled; // opposite to previous state!
        debug!(
            "Set/change custom topic: was it enabled? {:?}",
            just_enabled
        );
        let new_topic_option = {
            if just_enabled {
                None
            } else {
                Some(String::from(entry.common().channel_def.generated_topic()))
            }
        };
        debug!("New topic option: {:?}", new_topic_option);

        match new_topic_option {
            Some(t) => {
                debug!("Enable custom topic");
                entry.common_mut().custom_topic = Some(t)
            }
            None => {
                debug!("Disable custom topic");
                entry.common_mut().custom_topic = None
            }
        }
    }
    ui.add_enabled_ui(was_custom_topic_enabled, |ui| {
        if let Some(custom_topic) = &mut entry.common_mut().custom_topic {
            if ui.text_edit_singleline(custom_topic).changed() {
                update_channel_definition(entry, tether_agent);
            }
        } else {
            ui.label(entry.common().channel_def.generated_topic());
        }
    });

    common_edit_midi_mapping(ui, entry);

    ui.collapsing("Publish options", |ui| {
        ui.group(|ui| {
            ui.label("QOS level");
            ui.radio_value(
                &mut entry.common_mut().qos,
                QoS::AtLeastOnce,
                "0: At least once",
            )
            .on_hover_text("Fastest, no delivery guarrantees");
            ui.radio_value(
                &mut entry.common_mut().qos,
                QoS::AtMostOnce,
                "1: At most once",
            )
            .on_hover_text("Ensure delivery, duplicates possible");
            ui.radio_value(
                &mut entry.common_mut().qos,
                QoS::ExactlyOnce,
                "2: Exactly once",
            )
            .on_hover_text("Slowest, guarranteed once-only delivery");
        });
        ui.group(|ui| {
            ui.checkbox(&mut entry.common_mut().retain, "Retain?");
        });
    });
}

fn update_channel_definition<T: Serialize>(
    entry: &mut impl CustomWidget<T>,
    tether_agent: &mut TetherAgent,
) {
    debug!("Will update channel definition");
    debug!("QOS level: {}", entry.common().qos as i32);
    debug!("Retain: {}", entry.common().retain);

    entry.common_mut().channel_def = ChannelSenderDefBuilder::new(&entry.common().channel_name)
        .qos(Some(entry.common().qos))
        .retain(Some(entry.common().retain))
        .override_topic(entry.common().custom_topic.as_deref())
        .build(tether_agent);
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
