use egui::Ui;
use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use crate::{
    gui::{
        common_editable_values, common_in_use_heading, common_save_button, common_send,
        common_send_button,
    },
    midi_mapping::MidiMapping,
};

use super::{Common, CustomWidget, View};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyWidget {
    common: Common,
    value: (),
}

impl EmptyWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        EmptyWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: (),
        }
    }
}

impl CustomWidget<()> for EmptyWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &() {
        &self.value
    }

    fn value_mut(&mut self) -> &mut () {
        &mut self.value
    }
}

impl View for EmptyWidget {
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

        if common_send_button(ui, self, false).clicked() {
            common_send(self, tether_agent);
        };
    }
    fn render_editing(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_editable_values(ui, self, tether_agent);
        common_save_button(ui, self);
    }
}
