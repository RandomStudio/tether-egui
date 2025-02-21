use egui::{Slider, Ui};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use tether_agent::TetherAgent;

use crate::{
    gui::widget_view::{
        common_editable_values, common_in_use_heading, common_save_button, common_send,
        common_send_button,
    },
    midi_mapping::MidiMapping,
};

use super::{Common, CustomWidget, View};

const SENSIBLE_MIN: f64 = -100000.;
const SENSIBLE_MAX: f64 = 100000.;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberWidget {
    common: Common,
    value: f64,
    range_min: f64,
    range_max: f64,
    should_round: bool,
    #[serde(default = "default_step_size")]
    step_size: f64,
}

fn default_step_size() -> f64 {
    1.0
}

#[allow(clippy::too_many_arguments)]
impl NumberWidget {
    pub fn new(
        name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        value: f64,
        range: RangeInclusive<f64>,
        round_off: bool,
        agent: &mut TetherAgent,
    ) -> Self {
        NumberWidget {
            common: Common::new(name, description, plug_name, custom_topic, agent),
            value,
            range_min: *range.start(),
            range_max: *range.end(),
            should_round: round_off,
            step_size: {
                if round_off {
                    1.0
                } else {
                    (*range.start() - *range.end()).abs() / 100.
                }
            },
        }
    }

    pub fn range(&self) -> RangeInclusive<f64> {
        self.range_min..=self.range_max
    }

    pub fn should_round(&self) -> bool {
        self.should_round
    }
}

impl CustomWidget<f64> for NumberWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &f64 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f64 {
        &mut self.value
    }
}

impl View for NumberWidget {
    fn render_in_use(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_in_use_heading(ui, self);

        let &min = self.range().start();
        let &max = self.range().end();

        if ui
            .add(
                Slider::new(&mut self.value, min..=max)
                    .clamp_to_range(false)
                    .step_by(self.step_size),
            )
            .changed()
            && self.common().auto_send
        {
            if self.should_round {
                // Round off the value (internally f64)
                *self.value_mut() = self.value().round();
                // Make sure we convert to integer explicity before sending
                let value = *self.value() as i64;
                let payload = rmp_serde::to_vec(&value).expect("failed to serialised");
                tether_agent
                    .publish(&self.common().plug, Some(&payload))
                    .expect("failed to publish");
            } else {
                // No rounding, just encode and publish
                common_send(self, tether_agent);
            }
        };
        ui.horizontal(|ui| {
            ui.small(format!(
                "Range: {}-{}",
                self.range().start(),
                self.range().end()
            ));
            if let Some(midi) = &self.common().midi_mapping {
                match midi {
                    MidiMapping::Learning => {}
                    MidiMapping::Set(mapping) => {
                        ui.label(format!(
                            "MIDI mapped: ch {} cc {}",
                            mapping.channel, mapping.controller_or_note
                        ));
                    }
                }
            }
        });

        if common_send_button(ui, self, true).clicked() {
            common_send(self, tether_agent);
        };
    }

    fn render_editing(&mut self, ui: &mut Ui, tether_agent: &mut TetherAgent) {
        common_editable_values(ui, self, tether_agent);
        ui.collapsing("Range", |ui| {
            // This trickery is needed so that we artifically restrict slider ranges to
            // a sensible range while preserving types
            // let (min, max) = (T::from_f64(SENSIBLE_MIN), T::from_f64(SENSIBLE_MAX));

            ui.label("Min");
            if ui
                .add(Slider::new(
                    &mut self.range_min,
                    SENSIBLE_MIN..=SENSIBLE_MAX,
                ))
                .changed()
            {};
            ui.label("Max");
            ui.add(Slider::new(
                &mut self.range_max,
                SENSIBLE_MIN..=SENSIBLE_MAX,
            ));

            ui.label("StepSize");
            let max_step = self.range_max - self.range_min;
            ui.add(Slider::new(&mut self.step_size, 0.0..=max_step));
        });

        common_save_button(ui, self, tether_agent);
    }
}
