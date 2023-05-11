use serde::{Deserialize, Serialize};
use tether_agent::{PlugDefinition, TetherAgent};

// Re-export modules
pub mod boolean;
pub mod colours;
pub mod empty;
pub mod numbers;
pub mod point;

pub trait CustomWidget<T> {
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
