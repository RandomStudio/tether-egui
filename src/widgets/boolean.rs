use egui::Ui;
use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use crate::{
    midi_mapping::MidiMapping,
    ui::{
        common_editable_values, common_in_use_heading, common_save_button, common_send,
        common_send_button,
    },
};

use super::{Common, CustomWidget, View};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoolWidget {
    common: Common,
    value: bool,
}

impl BoolWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        init_state: bool,
        agent: &TetherAgent,
    ) -> Self {
        BoolWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: init_state,
        }
    }
}

impl CustomWidget<bool> for BoolWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &bool {
        &self.value
    }

    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl View for BoolWidget {
    fn render_in_use(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_in_use_heading(ui, self);

        if let Some(midi) = &self.common().midi_mapping {
            match midi {
                MidiMapping::Learning => {}
                MidiMapping::Set(mapping) => {
                    ui.label(format!(
                        "MIDI mapped: toggle on ch {} note {}",
                        mapping.channel, mapping.controller_or_note
                    ));
                }
            }
        }

        let checked = *self.value();
        if ui
            .checkbox(
                self.value_mut(),
                format!("State: {}", {
                    if checked {
                        "TRUE"
                    } else {
                        "FALSE "
                    }
                }),
            )
            .clicked()
            && self.common().auto_send
            || common_send_button(ui, self, true).clicked()
        {
            common_send(self, tether_agent);
        }
    }

    fn render_editing(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_editable_values(ui, self, tether_agent);
        common_save_button(ui, self);
    }
}
