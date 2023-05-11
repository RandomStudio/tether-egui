use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use super::{Common, CustomWidget};

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
        rgba: ColourRGBA8,
        agent: &TetherAgent,
    ) -> Self {
        ColourWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: rgba,
        }
    }
}

impl CustomWidget<ColourRGBA8> for ColourWidget<ColourRGBA8> {
    fn common(&self) -> &Common {
        &self.common
    }
    fn value(&self) -> &ColourRGBA8 {
        &self.value
    }

    fn value_mut(&mut self) -> &mut ColourRGBA8 {
        &mut self.value
    }
}