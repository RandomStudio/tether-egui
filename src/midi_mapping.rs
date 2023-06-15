use egui::{
    emath::{Numeric, Real},
    remap,
};
use log::debug;
use serde::{Deserialize, Serialize};
use tether_agent::{mqtt::Message, TetherAgent};

use crate::widgets::{numbers::NumberWidget, CustomWidget};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MidiMapped {
    pub channel: u8,
    pub controller: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TetherControlChangePayload {
    pub channel: u8,
    pub controller: u8,
    pub value: u8,
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
                agent.create_input_plug("controlChange", None, Some("+/+/controlChange"));
        }
        MidiSubscriber {}
    }

    pub fn get_controller_message(
        &self,
        plug_name: &String,
        message: &Message,
    ) -> Option<TetherControlChangePayload> {
        if plug_name == "controlChange" {
            debug!(
                "this is a Tether MIDI control change message: {} :: {:?}",
                plug_name, message
            );
            let bytes = message.payload();
            let payload: TetherControlChangePayload =
                rmp_serde::from_slice(bytes).expect("failed to decode payload");
            Some(payload)
        } else {
            None
        }
    }
}

pub fn update_widget_if_controllable(
    entry: &mut NumberWidget,
    cc_message: &TetherControlChangePayload,
) -> bool {
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
                    controller: *controller,
                }));
                false
            }
            MidiMapping::Set(mapping) => {
                if mapping.channel == *channel && mapping.controller == *controller {
                    debug!("Message matches MIDI mapping, should update");
                    let output_range = entry.range();
                    // let input_range = T::from_f64(0.)..=T::from_f64(127.0);
                    let input_range = 0. ..=127.;
                    let midi_value_number = *value as f64;
                    let remapped_value = remap(midi_value_number, input_range, output_range);
                    let v = entry.value_mut();
                    *v = remapped_value;
                    true
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}
