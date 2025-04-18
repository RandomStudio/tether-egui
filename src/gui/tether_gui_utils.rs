use serde::{Deserialize, Serialize};
use tether_agent::{TetherAgent, TetherAgentBuilder};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditableTetherSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub role: String,
    pub id: Option<String>,

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
            id: None,
        }
    }
}

impl From<EditableTetherSettings> for TetherAgentBuilder {
    fn from(editable: EditableTetherSettings) -> Self {
        TetherAgentBuilder::new(&editable.role)
            .id(editable.id.as_deref())
            .host(Some(&editable.host))
            .port(Some(editable.port))
            .username(Some(&editable.username))
            .password(Some(&editable.password))
            .auto_connect(false)
    }
}

pub fn unconnected_tether_agent(options: &TetherAgentBuilder) -> TetherAgent {
    options
        .clone()
        .auto_connect(false)
        .build()
        .expect("Failed to initialise (not connect) new Tether Agent")
}
