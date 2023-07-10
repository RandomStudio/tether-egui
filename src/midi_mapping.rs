use egui::remap;
use log::debug;
use serde::{Deserialize, Serialize};
use tether_agent::{mqtt::Message, PlugOptionsBuilder, TetherAgent};

use crate::{
    gui::project_builder::common_send,
    widgets::{boolean::BoolWidget, numbers::NumberWidget, CustomWidget},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MidiMapped {
    pub channel: u8,
    pub controller_or_note: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TetherControlChangePayload {
    pub channel: u8,
    pub controller: u8,
    pub value: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TetherNotePayload {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}

pub enum MidiMessage {
    ControlChange(TetherControlChangePayload),
    Note(TetherNotePayload),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MidiMapping {
    Learning,
    Set(MidiMapped),
}

pub struct MidiSubscriber {}

impl MidiSubscriber {
    /// Subscribe to all Tether MIDI control change messages
    pub fn new(agent: &TetherAgent) -> Self {
        if agent.is_connected() {
            let _midi_controllers_plug =
                PlugOptionsBuilder::create_input("controlChange").build(agent);
            let _midi_notes_plug = PlugOptionsBuilder::create_input("notesOn").build(agent);
        }
        MidiSubscriber {}
    }

    pub fn get_midi_message(&self, plug_name: &String, message: &Message) -> Option<MidiMessage> {
        match plug_name.as_str() {
            "controlChange" => {
                debug!(
                    "This is a Tether MIDI control change message: {} :: {:?}",
                    plug_name, message
                );
                let bytes = message.payload();
                let payload: TetherControlChangePayload =
                    rmp_serde::from_slice(bytes).expect("failed to decode payload");
                Some(MidiMessage::ControlChange(payload))
            }
            "notesOn" => {
                debug!(
                    "This is a Tether MIDI note on message: {} :: {:?}",
                    plug_name, message
                );
                let bytes = message.payload();
                let payload: TetherNotePayload =
                    rmp_serde::from_slice(bytes).expect("failed to decode payload");
                Some(MidiMessage::Note(payload))
            }
            _ => None,
        }
    }
}

pub fn update_widget_if_controllable(
    entry: &mut NumberWidget,
    cc_message: &TetherControlChangePayload,
    tether_agent: &TetherAgent,
) {
    if let Some(midi_mapping) = &entry.common().midi_mapping {
        let TetherControlChangePayload {
            channel,
            controller,
            value,
        } = cc_message;
        match midi_mapping {
            MidiMapping::Learning => {
                entry.common_mut().midi_mapping = Some(MidiMapping::Set(MidiMapped {
                    channel: *channel,
                    controller_or_note: *controller,
                }));
            }
            MidiMapping::Set(mapping) => {
                if mapping.channel == *channel && mapping.controller_or_note == *controller {
                    debug!("Message matches MIDI mapping, should update");
                    let output_range = entry.range();
                    let should_round = entry.should_round();
                    // let input_range = T::from_f64(0.)..=T::from_f64(127.0);
                    let input_range = 0. ..=127.;
                    let midi_value_number = *value as f64;
                    let remapped_value = remap(midi_value_number, input_range, output_range);
                    let v = entry.value_mut();
                    *v = if should_round {
                        remapped_value.round()
                    } else {
                        remapped_value
                    };
                    common_send(entry, tether_agent);
                }
            }
        }
    }
}

pub fn send_if_midi_note<T: Serialize>(
    entry: &mut impl CustomWidget<T>,
    note_message: &TetherNotePayload,
) -> bool {
    if let Some(midi_mapping) = &entry.common().midi_mapping {
        let TetherNotePayload {
            channel,
            note,
            velocity: _,
        } = note_message;
        match midi_mapping {
            MidiMapping::Learning => {
                entry.common_mut().midi_mapping = Some(MidiMapping::Set(MidiMapped {
                    channel: *channel,
                    controller_or_note: *note,
                }));
                false
            }
            MidiMapping::Set(mapping) => {
                mapping.channel == *channel && mapping.controller_or_note == *note
            }
        }
    } else {
        false
    }
}

pub fn toggle_if_midi_note(
    entry: &mut BoolWidget,
    note_message: &TetherNotePayload,
    tether_agent: &TetherAgent,
) {
    if let Some(midi_mapping) = &entry.common().midi_mapping {
        let TetherNotePayload {
            channel,
            note,
            velocity: _,
        } = note_message;
        match midi_mapping {
            MidiMapping::Learning => {
                entry.common_mut().midi_mapping = Some(MidiMapping::Set(MidiMapped {
                    channel: *channel,
                    controller_or_note: *note,
                }));
            }
            MidiMapping::Set(mapping) => {
                if mapping.channel == *channel && mapping.controller_or_note == *note {
                    *entry.value_mut() = !*entry.value();
                    common_send(entry, tether_agent);
                }
            }
        }
    }
}
