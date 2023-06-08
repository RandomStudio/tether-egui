use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MidiMapped {
    pub channel: u8,
    pub controller: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MidiMapping {
    Learning,
    Set(MidiMapped),
}

pub struct MidiSubscriber {}
