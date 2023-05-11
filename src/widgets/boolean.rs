use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use super::{Common, CustomWidget};

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

impl CustomWidget<bool> for BoolWidget {
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
