use egui::Ui;
use serde::{Deserialize, Serialize};
use tether_agent::{PlugDefinition, TetherAgent};

use crate::Model;

use self::{
    boolean::BoolWidget,
    colours::{ColourRGBA8, ColourWidget},
    empty::EmptyWidget,
    generic::GenericJSONWidget,
    numbers::NumberWidget,
    point::Point2DWidget,
};

// Re-export modules
pub mod boolean;
pub mod colours;
pub mod empty;
pub mod generic;
pub mod numbers;
pub mod point;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The different possible Widget entries. Serialisable because
/// these can be saved/loaded to/from disk
pub enum WidgetEntry {
    FloatNumber(NumberWidget<f64>),
    WholeNumber(NumberWidget<i64>),
    Colour(ColourWidget<ColourRGBA8>),
    Bool(BoolWidget),
    Empty(EmptyWidget),
    Point2D(Point2DWidget),
    Generic(GenericJSONWidget),
}

pub trait CustomWidget<T> {
    fn common(&self) -> &Common;
    fn common_mut(&mut self) -> &mut Common;
    fn value(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
    fn is_edit_mode(&self) -> bool {
        self.common().is_edit_mode
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Common {
    pub name: String,
    pub description: String,
    pub plug: PlugDefinition,
    #[serde(skip)]
    is_edit_mode: bool,
    #[serde(skip)]
    pub use_custom_topic: bool,
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
            is_edit_mode: true,
            use_custom_topic: false,
        }
    }

    pub fn is_edit_mode(&self) -> bool {
        self.is_edit_mode
    }

    pub fn set_edit_mode(&mut self, value: bool) {
        self.is_edit_mode = value
    }
}

pub trait View {
    fn render_in_use(&mut self, ctx: &egui::Context, index: usize, tether_agent: &TetherAgent);
    fn render_editing(&mut self, ctx: &egui::Context, index: usize);
}
