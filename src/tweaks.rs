pub trait Tweak {
  fn name(&self) -> &str;
  fn description(&self) -> &str;
}

pub struct NumberTweak {
  pub name: String,
  pub value: f32,
  pub range: Option<(f32, f32)>,
  pub description: String
}

impl Tweak for NumberTweak {
  fn name(&self) -> &str {
    &self.name
  }
  fn description(&self) -> &str {
   &self.description
  }
}