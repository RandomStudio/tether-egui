use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use tether_agent::{PlugDefinition, TetherAgent};

pub trait Widget<T> {
    fn common(&self) -> &Common;
    fn value(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Common {
    pub name: String,
    pub description: String,
    pub plug: PlugDefinition,
}

impl Common {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        Common {
            name: String::from(widget_name),
            description: {
                if let Some(d) = description {
                    String::from(d)
                } else {
                    String::from("no description provided")
                }
            },
            plug: agent
                .create_output_plug(plug_name, None, custom_topic)
                .unwrap(),
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberWidget<T> {
    common: Common,
    value: T,
    range: RangeInclusive<f32>,
}

impl NumberWidget<f32> {
    pub fn new(
        name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        value: f32,
        range: Option<RangeInclusive<f32>>,
        agent: &TetherAgent,
    ) -> Self {
        NumberWidget {
            common: Common::new(name, description, plug_name, custom_topic, agent),
            value,
            range: range.unwrap_or(0. ..=1.),
        }
    }

    pub fn range(&self) -> (f32, f32) {
        (*self.range.start(), *self.range.end())
    }
}

impl Widget<f32> for NumberWidget<f32> {
    fn common(&self) -> &Common {
        &self.common
    }
    fn value(&self) -> &f32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
}

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

impl Widget<ColourRGBA8> for ColourWidget<ColourRGBA8> {
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

impl Widget<bool> for BoolWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn value(&self) -> &bool {
        &self.value
    }

    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}
