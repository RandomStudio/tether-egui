use log::{error, info};
use serde::{Deserialize, Serialize};
use tether_agent::{TetherAgent, TetherAgentOptionsBuilder};
use tether_utils::tether_topics::{insights::Insights, TopicOptions};

use crate::{midi_mapping::MidiSubscriber, model::Model};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditableTetherSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub role: String,
    pub id: String,

    #[serde(skip)]
    pub is_editing: bool,
    #[serde(skip)]
    pub was_changed: bool,
}

impl Default for EditableTetherSettings {
    fn default() -> Self {
        EditableTetherSettings {
            is_editing: false,
            was_changed: false,
            host: "localhost".into(),
            port: 1883,
            username: "tether".into(),
            password: "sp_ceB0ss!".into(),
            role: "gui".into(),
            id: "any".into(),
        }
    }
}

impl From<EditableTetherSettings> for TetherAgentOptionsBuilder {
    fn from(editable: EditableTetherSettings) -> Self {
        TetherAgentOptionsBuilder::new(&editable.role)
            .id(Some(&editable.id))
            .host(Some(&editable.host))
            .port(Some(editable.port))
            .username(Some(&editable.username))
            .password(Some(&editable.password))
            .auto_connect(false)
    }
}

pub fn unconnected_tether_agent(options: &TetherAgentOptionsBuilder) -> TetherAgent {
    options
        .clone()
        .auto_connect(false)
        .build()
        .expect("Failed to initialise (not connect) new Tether Agent")
}

pub fn attempt_new_tether_connection(model: &mut Model) {
    match &model.tether_agent.connect() {
        Ok(()) => {
            info!("Connected Tether Agent OK");
            // model.project.tether_settings.was_changed = true;
            model.insights = Some(Insights::new(
                &TopicOptions {
                    topic: model.monitor_topic.clone(),
                    sampler_interval: 1000,
                    graph_enable: false,
                },
                &model.tether_agent,
            ));
            model.midi_handler = Some(MidiSubscriber::new(&model.tether_agent));
        }
        Err(e) => {
            error!("Failed to connect Tether Agent: {}", e);
        }
    }
}
