use serde::Serialize;
use std::ops::RangeInclusive;
use tether_agent::{PlugDefinition, TetherAgent};

pub trait Tweak {
    fn common(&self) -> &Common;
}

pub struct Common {
    pub name: String,
    pub description: String,
    pub plug: PlugDefinition,
}

impl Common {
    pub fn new(
        tweak_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        Common {
            name: String::from(tweak_name),
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
// pub fn topic(&self, agent: &TetherAgent) -> String {
//     let (role, id) = agent.description();
//     format!("{}/{}/{}", role, id, self.plug_name)
// }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]

pub struct NumberTweak {
    #[serde(skip)]
    common: Common,
    value: f32,
    range: RangeInclusive<f32>,
}

impl NumberTweak {
    pub fn new(
        name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        value: f32,
        range: Option<RangeInclusive<f32>>,
        agent: &TetherAgent,
    ) -> Self {
        NumberTweak {
            common: Common::new(name, description, plug_name, custom_topic, agent),
            value,
            range: range.unwrap_or(0. ..=1.),
        }
    }
    pub fn value(&self) -> f32 {
        self.value
    }
    pub fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }

    pub fn range(&self) -> (f32, f32) {
        (*self.range.start(), *self.range.end())
    }
}

impl Tweak for NumberTweak {
    fn common(&self) -> &Common {
        &self.common
    }
}

type ColourRGBA8 = [u8; 4];

pub struct ColourTweak {
    common: Common,
    value: ColourRGBA8,
}

impl ColourTweak {
    pub fn new(
        tweak_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        rgba: (u8, u8, u8, u8),
        agent: &TetherAgent,
    ) -> Self {
        let (r, g, b, a) = rgba;
        ColourTweak {
            common: Common::new(tweak_name, description, plug_name, custom_topic, agent),
            value: [r, g, b, a],
        }
    }

    pub fn value(&self) -> ColourRGBA8 {
        self.value
    }

    pub fn value_mut(&mut self) -> &mut ColourRGBA8 {
        &mut self.value
    }
}

impl Tweak for ColourTweak {
    fn common(&self) -> &Common {
        &self.common
    }
}
