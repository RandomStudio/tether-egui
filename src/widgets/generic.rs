use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use super::{Common, CustomWidget};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericJSON {
    common: Common,
    value: String,
}

impl GenericJSON {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        GenericJSON {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: "{\"answer\":42}".into(),
        }
    }
}

impl CustomWidget<String> for GenericJSON {
    fn common(&self) -> &Common {
        &self.common
    }
    fn value(&self) -> &String {
        &self.value
    }

    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}
