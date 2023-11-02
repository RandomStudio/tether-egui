use serde::{Deserialize, Serialize};
use tether_agent::{TetherAgent, TetherAgentOptionsBuilder};

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
