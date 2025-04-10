use log::*;
use serde::{Deserialize, Serialize};
use tether_agent::{ChannelDefinition, ChannelOptionsBuilder, TetherAgent};

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
    pub channel: ChannelDefinition,
    pub midi_mapping: Option<MidiMapping>,

    // The fields below are never used in on-disk versions,
    // only in-memory state
    #[serde(skip)]
    is_edit_mode: bool,
    #[serde(skip)]
    pub custom_topic: Option<String>,
    #[serde(skip)]
    pub channel_name: String,
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
    // String::from(full_name.to_lowercase().replace(' ', "").trim())
    let parts = full_name.split(" ").collect::<Vec<&str>>();
    let mut s = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            s.push_str(&part.to_lowercase());
        } else {
            s.push_str(&part[0..1].to_uppercase());
            s.push_str(&part[1..].to_lowercase());
        }
    }
    s
}

impl Common {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        channel_name: &str,
        custom_topic: Option<&str>,
        agent: &mut TetherAgent,
    ) -> Self {
        debug!("New Widget: with Custom topic? {:?}", custom_topic);
        let channel = match custom_topic {
            Some(topic) => ChannelOptionsBuilder::create_sender(channel_name)
                .topic(Some(topic))
                .build(agent)
                .expect("failed to create output"),

            None => ChannelOptionsBuilder::create_sender(channel_name)
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
            channel,
            is_edit_mode: true,
            channel_name: shortened_name(widget_name),
            auto_send: true,
            midi_mapping: None,
            qos: Qos::AtMostOnce,
            retain: false,
            custom_topic: custom_topic.map(String::from),
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
