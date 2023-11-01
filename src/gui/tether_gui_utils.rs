use tether_agent::{TetherAgent, TetherAgentOptionsBuilder};

use crate::project::TetherSettingsInProject;

#[derive(Clone)]
pub struct EditableTetherSettings {
    pub is_editing: bool,
    pub was_changed: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub role: String,
    pub id: String,
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

impl From<&TetherSettingsInProject> for EditableTetherSettings {
    fn from(project: &TetherSettingsInProject) -> Self {
        let default_editable_settings = EditableTetherSettings::default();
        let TetherSettingsInProject {
            host,
            port,
            username,
            password,
            role,
            id,
        } = project;

        let host = if host.is_some() {
            host.clone().unwrap()
        } else {
            default_editable_settings.host.clone()
        };

        let username = if username.is_some() {
            username.clone().unwrap()
        } else {
            default_editable_settings.username.clone()
        };

        let password = if password.is_some() {
            password.clone().unwrap()
        } else {
            default_editable_settings.password.clone()
        };

        let role = if role.is_some() {
            role.clone().unwrap()
        } else {
            default_editable_settings.role.clone()
        };

        let id = if id.is_some() {
            id.clone().unwrap()
        } else {
            default_editable_settings.id.clone()
        };

        EditableTetherSettings {
            is_editing: false,
            was_changed: false,
            host,
            port: port.unwrap_or(default_editable_settings.port),
            username,
            password,
            role,
            id,
        }
    }
}

impl From<&TetherSettingsInProject> for TetherAgentOptionsBuilder {
    fn from(project: &TetherSettingsInProject) -> Self {
        let project = project.clone();

        TetherAgentOptionsBuilder::new(&project.role.unwrap_or("gui".into()))
            .id(project.id.as_deref())
            .host(project.host.as_deref())
            .port(project.port)
            .username(project.username.as_deref())
            .password(project.password.as_deref())
            .auto_connect(false)
    }
}

impl From<&EditableTetherSettings> for TetherAgentOptionsBuilder {
    fn from(editable: &EditableTetherSettings) -> Self {
        TetherAgentOptionsBuilder::new(&editable.role)
            .id(Some(&editable.id))
            .host(Some(&editable.host))
            .port(Some(editable.port))
            .username(Some(&editable.username))
            .password(Some(&editable.password))
            .auto_connect(false)
    }
}

pub fn init_new_tether_agent(options: &TetherAgentOptionsBuilder) -> TetherAgent {
    options
        .clone()
        .auto_connect(false)
        .build()
        .expect("failed to init (not connect) new Tether Agent")
}
