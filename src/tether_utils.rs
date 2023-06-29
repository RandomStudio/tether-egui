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
            username: "".into(),
            password: "".into(),
            role: "any".into(),
            id: "any".into(),
        }
    }
}

impl From<&TetherAgentOptionsBuilder> for EditableTetherSettings {
    fn from(options: &TetherAgentOptionsBuilder) -> Self {
        let TetherAgentOptionsBuilder {
            role,
            id,
            host,
            port,
            username,
            password,
            ..
        } = options.clone();
        let default_editable_settings = EditableTetherSettings::default();
        EditableTetherSettings {
            is_editing: false,
            was_changed: false,
            host: host.unwrap_or(default_editable_settings.host),
            port: port.unwrap_or(default_editable_settings.port),
            username: username.unwrap_or(default_editable_settings.username),
            password: password.unwrap_or(default_editable_settings.password),
            role: role.into(),
            id: id.unwrap_or(default_editable_settings.id),
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

        let role = role.clone();

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
    fn from(project_settings: &TetherSettingsInProject) -> Self {
        let role = if project_settings.role.is_some() {
            project_settings.role.clone().unwrap()
        } else {
            "gui".into()
        };
        TetherAgentOptionsBuilder {
            role,
            id: project_settings.id.clone(),
            host: project_settings.host.clone(),
            port: project_settings.port,
            username: project_settings.username.clone(),
            password: project_settings.password.clone(),
            auto_connect: false,
        }
    }
}

impl From<&EditableTetherSettings> for TetherAgentOptionsBuilder {
    fn from(editable: &EditableTetherSettings) -> Self {
        let EditableTetherSettings {
            role,
            id,
            host,
            port,
            username,
            password,
            ..
        } = editable.clone();
        TetherAgentOptionsBuilder {
            role,
            id: Some(id),
            host: Some(host),
            port: Some(port),
            username: Some(username),
            password: Some(password),
            auto_connect: false,
        }
    }
}

pub fn init_new_tether_agent(options: &TetherAgentOptionsBuilder) -> TetherAgent {
    let tether_agent = options
        .clone()
        .auto_connect(false)
        .build()
        .expect("failed to init (not connect) new Tether Agent");

    tether_agent
}
