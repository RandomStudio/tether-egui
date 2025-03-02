use serde::{Deserialize, Serialize};
use tether_agent::{PlugDefinition, PlugOptionsBuilder, TetherAgent};

use crate::midi_mapping::MidiMapping;

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
    FloatNumber(NumberWidget),
    WholeNumber(NumberWidget),
    Colour(ColourWidget<ColourRGBA8>),
    Bool(BoolWidget),
    Empty(EmptyWidget),
    Point2D(Point2DWidget),
    Generic(GenericJSONWidget),
}

pub trait CustomWidget<T: Serialize> {
    fn common(&self) -> &Common;
    fn common_mut(&mut self) -> &mut Common;
    fn value(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Qos {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Represents user-defined options common to all Widgets
pub struct Common {
    pub name: String,
    pub description: String,
    pub plug: PlugDefinition,
    pub midi_mapping: Option<MidiMapping>,

    // The fields below are never used in on-disk versions,
    // only in-memory state
    #[serde(skip)]
    is_edit_mode: bool,
    #[serde(skip)]
    pub use_custom_topic: bool,
    #[serde(skip)]
    pub custom_topic: String,
    #[serde(skip)]
    pub plug_name: String,
    #[serde(skip, default = "default_qos")]
    pub qos: Qos,
    #[serde(skip)]
    pub retain: bool,

    #[serde(skip, default = "default_auto_send")]
    pub auto_send: bool,
}

fn default_auto_send() -> bool {
    true
}

fn default_qos() -> Qos {
    Qos::AtLeastOnce
}

pub fn shortened_name(full_name: &str) -> String {
    String::from(full_name.replace(' ', "_").trim())
}

impl Common {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &mut TetherAgent,
    ) -> Self {
        let plug = match custom_topic {
            Some(topic) => PlugOptionsBuilder::create_output(plug_name)
                .topic(Some(topic))
                .build(agent)
                .expect("failed to create output"),

            None => PlugOptionsBuilder::create_output(plug_name)
                .build(agent)
                .expect("failed to create output"),
        };

        Common {
            name: String::from(widget_name),
            description: {
                if let Some(d) = description {
                    String::from(d)
                } else {
                    String::from("no description provided")
                }
            },
            plug,
            is_edit_mode: true,
            plug_name: shortened_name(widget_name),
            use_custom_topic: false,
            auto_send: true,
            midi_mapping: None,
            qos: Qos::AtMostOnce,
            retain: false,
            custom_topic: String::from(""),
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
    fn render_editing(&mut self, ui: &mut egui::Ui, tether_agent: &mut TetherAgent);
    fn render_in_use(&mut self, ui: &mut egui::Ui, tether_agent: &TetherAgent);
}

// pub trait MidiControllable {
//     fn handle_midi_message(&mut self, )
// }
