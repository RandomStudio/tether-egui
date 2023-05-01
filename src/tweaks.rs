use std::ops::RangeInclusive;

pub trait Tweak {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub struct NumberTweak {
    name: String,
    description: String,
    value: f32,
    range: RangeInclusive<f32>,
}

impl NumberTweak {
    pub fn new(
        name: String,
        description: String,
        value: f32,
        range: Option<RangeInclusive<f32>>,
    ) -> Self {
        NumberTweak {
            name,
            description,
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
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
}

type ColourRGBA8 = (u8, u8, u8, f32);

pub struct ColourTweak {
    name: String,
    description: String,
    value: ColourRGBA8,
}

impl ColourTweak {
    pub fn new(name: String, description: String, rgb: (u8, u8, u8), alpha: f32) -> Self {
        let (r, g, b) = rgb;
        ColourTweak {
            name,
            description,
            value: (r, g, b, alpha),
        }
    }

    pub fn value(&self) -> ColourRGBA8 {
        self.value
    }
}

impl Tweak for ColourTweak {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
}
