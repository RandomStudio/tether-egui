use egui::emath::Numeric;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use tether_agent::TetherAgent;

use super::{Common, CustomWidget};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberWidget<T> {
    common: Common,
    value: T,
    range: RangeInclusive<T>,
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
            range,
        }
    }

    pub fn range(&self) -> (T, T) {
        (*self.range.start(), *self.range.end())
    }
}

impl<T: Numeric> CustomWidget<T> for NumberWidget<T> {
    fn common(&self) -> &Common {
        &self.common
    }
    fn value(&self) -> &T {
        &self.value
    }
    fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
