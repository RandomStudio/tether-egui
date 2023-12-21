use egui::{Color32, Ui};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tether_agent::TetherAgent;

use crate::{
    gui::widget_view::{
        common_editable_values, common_in_use_heading, common_save_button, common_send_button,
    },
    midi_mapping::MidiMapping,
};

use super::{Common, CustomWidget, View};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericJSONWidget {
    common: Common,
    value: String,
    #[serde(skip, default = "assume_valid")]
    is_valid_json: bool,
}

/// Since valid state is not known on "load", we will
/// assume JSON is valid until parsed otherwise
fn assume_valid() -> bool {
    true
}

impl GenericJSONWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        GenericJSONWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: "{\"answer\":42}".into(),
            is_valid_json: true,
        }
    }

    pub fn publish_from_json_string(&self, tether_agent: &TetherAgent) {
        match serde_json::from_str::<serde_json::Value>(&self.value) {
            Ok(encoded) => {
                let payload = rmp_serde::to_vec_named(&encoded).expect("failed to encode msgpack");
                match tether_agent.publish(&self.common().plug, Some(&payload)) {
                    Ok(()) => debug!("Send OK"),
                    Err(_) => error!("Failed to send; connected? {}", tether_agent.is_connected()),
                }
            }
            Err(e) => {
                error!("Could not serialise String -> JSON; error: {}", e);
            }
        }
    }
}

impl CustomWidget<String> for GenericJSONWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &String {
        &self.value
    }

    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl View for GenericJSONWidget {
    fn render_editing(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_editable_values(ui, self, tether_agent);
        common_save_button(ui, self, tether_agent);
    }

    fn render_in_use(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_in_use_heading(ui, self);

        if let Some(midi) = &self.common().midi_mapping {
            match midi {
                MidiMapping::Learning => {}
                MidiMapping::Set(mapping) => {
                    ui.label(format!(
                        "MIDI mapped: send on ch {} note {}",
                        mapping.channel, mapping.controller_or_note
                    ));
                }
            }
        }

        if ui.text_edit_multiline(self.value_mut()).changed() {
            self.is_valid_json = !serde_json::from_str::<Value>(self.value()).is_err();
        }
        if self.is_valid_json {
            ui.colored_label(Color32::LIGHT_GREEN, "Valid JSON");
        } else {
            ui.colored_label(Color32::RED, "Not valid JSON");
        }

        if common_send_button(ui, self, false).clicked() {
            self.publish_from_json_string(tether_agent);
        }
    }
}
