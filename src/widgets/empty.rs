use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use super::{Common, CustomWidget};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyWidget {
    common: Common,
    value: (),
}

impl EmptyWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        EmptyWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: (),
        }
    }
}

impl CustomWidget<()> for EmptyWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn value(&self) -> &() {
        &self.value
    }

    fn value_mut(&mut self) -> &mut () {
        &mut self.value
    }
}
