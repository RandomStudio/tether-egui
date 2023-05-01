use std::ops::RangeInclusive;

pub trait Tweak {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn plug_name(&self) -> &str;
}

pub struct Common {
    pub name: String,
    pub description: String,
    pub plug_name: String,
}

impl Common {
    pub fn new(name: &str, description: Option<&str>, plug_name: Option<&str>) -> Self {
        Common {
            name: String::from(name),
            description: {
                if let Some(d) = description {
                    String::from(d)
                } else {
                    String::from("no description provided")
                }
            },
            plug_name: {
                if let Some(p) = plug_name {
                    String::from(p)
                } else {
                    String::from(name)
                }
            },
        }
    }
}

impl Tweak for Common {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn description(&self) -> &str {
        self.description.as_str()
    }
    fn plug_name(&self) -> &str {
        self.plug_name.as_str()
    }
}

pub struct NumberTweak {
    common: Common,
    value: f32,
    range: RangeInclusive<f32>,
}

impl NumberTweak {
    pub fn new(
        name: &str,
        description: Option<&str>,
        plug_name: Option<&str>,
        value: f32,
        range: Option<RangeInclusive<f32>>,
    ) -> Self {
        NumberTweak {
            common: Common::new(name, description, plug_name),
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
    fn name(&self) -> &str {
        &self.common.name()
    }
    fn description(&self) -> &str {
        self.common.description()
    }
    fn plug_name(&self) -> &str {
        self.common.plug_name()
    }
}

type ColourRGBA8 = [u8; 4];

pub struct ColourTweak {
    common: Common,
    value: ColourRGBA8,
}

impl ColourTweak {
    pub fn new(
        name: &str,
        description: Option<&str>,
        plug_name: Option<&str>,
        rgba: (u8, u8, u8, u8),
    ) -> Self {
        let (r, g, b, a) = rgba;
        ColourTweak {
            common: Common::new(name, description, plug_name),
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
    fn name(&self) -> &str {
        self.common.name()
    }
    fn description(&self) -> &str {
        self.common.description()
    }
    fn plug_name(&self) -> &str {
        self.common.plug_name()
    }
}
