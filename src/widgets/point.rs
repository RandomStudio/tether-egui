use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use super::{Common, CustomWidget};

type Point2D = [f64; 2];

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point2DWidget {
    common: Common,
    value: Point2D,
}

impl Point2DWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        Point2DWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: [0., 0.],
        }
    }
}

impl CustomWidget<Point2D> for Point2DWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn value(&self) -> &Point2D {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Point2D {
        &mut self.value
    }
}
