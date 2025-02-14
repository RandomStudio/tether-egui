use egui::Ui;
use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use crate::{
    gui::widget_view::{
        common_editable_values, common_in_use_heading, common_save_button, common_send,
        common_send_button,
    },
    midi_mapping::MidiMapping,
};

use super::{Common, CustomWidget, View};

pub type ColourRGBA8 = [u8; 4];

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColourWidget<ColourRGBA8> {
    common: Common,
    value: ColourRGBA8,
}

impl ColourWidget<ColourRGBA8> {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &mut TetherAgent,
    ) -> Self {
        ColourWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: [255, 255, 255, 255],
        }
    }
}

impl CustomWidget<ColourRGBA8> for ColourWidget<ColourRGBA8> {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &ColourRGBA8 {
        &self.value
    }

    fn value_mut(&mut self) -> &mut ColourRGBA8 {
        &mut self.value
    }
}

impl View for ColourWidget<ColourRGBA8> {
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

        if ui
            .color_edit_button_srgba_unmultiplied(self.value_mut())
            .changed()
            && self.common().auto_send
            || common_send_button(ui, self, true).clicked()
        {
            common_send(self, tether_agent);
        }
    }

    fn render_editing(&mut self, ui: &mut Ui, tether_agent: &mut TetherAgent) {
        common_editable_values(ui, self, tether_agent);
        common_save_button(ui, self, tether_agent);
    }
}
