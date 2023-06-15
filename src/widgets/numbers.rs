use egui::{emath::Numeric, Slider, Ui};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::RangeInclusive};
use tether_agent::TetherAgent;

use crate::{
    midi_mapping::MidiMapping,
    ui::{
        common_editable_values, common_in_use_heading, common_save_button, common_send,
        common_send_button,
    },
};

use super::{Common, CustomWidget, View};

const SENSIBLE_MIN: f64 = -100000.;
const SENSIBLE_MAX: f64 = 100000.;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberWidget<T> {
    common: Common,
    value: T,
    range_min: T,
    range_max: T,
}

impl<T: Numeric> NumberWidget<T> {
    pub fn new(
        name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        value: T,
        range: RangeInclusive<T>,
        agent: &TetherAgent,
    ) -> Self {
        NumberWidget {
            common: Common::new(name, description, plug_name, custom_topic, agent),
            value,
            range_min: *range.start(),
            range_max: *range.end(),
        }
    }

    pub fn range(&self) -> RangeInclusive<T> {
        self.range_min..=self.range_max
    }
}

impl<T: Numeric + Serialize> CustomWidget<T> for NumberWidget<T> {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &T {
        &self.value
    }
    fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Numeric + Serialize + Display> View for NumberWidget<T> {
    fn render_in_use(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_in_use_heading(ui, self);

        let &min = self.range().start();
        let &max = self.range().end();

        if ui
            .add(Slider::new(&mut self.value, min..=max).clamp_to_range(false))
            .changed()
            && self.common().auto_send
        {
            debug!("Changed; send");
            common_send(self, tether_agent);
        };
        ui.small(format!(
            "Range: {}-{}",
            self.range().start(),
            self.range().end()
        ));

        if common_send_button(ui, self, true).clicked() {
            common_send(self, tether_agent);
        };
    }

    fn render_editing(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_editable_values(ui, self, tether_agent);
        ui.collapsing("Range", |ui| {
            // This trickery is needed so that we artifically restrict slider ranges to
            // a sensible range while preserving types
            let (min, max) = (T::from_f64(SENSIBLE_MIN), T::from_f64(SENSIBLE_MAX));

            ui.label("Min");
            if ui
                .add(Slider::new(&mut self.range_min, min..=max))
                .changed()
            {};
            ui.label("Max");
            ui.add(Slider::new(&mut self.range_max, min..=max));
        });

        // TODO move this into generic "common_midi" function
        if ui.button("Learn MIDI mapping").clicked() {
            self.common_mut().midi_mapping = Some(MidiMapping::Learning);
        }
        if let Some(midi) = &self.common().midi_mapping {
            match midi {
                MidiMapping::Learning => {
                    ui.label("Learning...");
                }
                MidiMapping::Set(mapping) => {
                    ui.label(format!("{:?}", mapping));
                }
            }
        }

        common_save_button(ui, self);
    }
}
