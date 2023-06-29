use std::fs;

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{tether_utils::EditableTetherSettings, widgets::WidgetEntry};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TetherSettingsInProject {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub id: Option<String>,
}

impl Default for TetherSettingsInProject {
    fn default() -> Self {
        TetherSettingsInProject {
            host: Some("127.0.0.1".into()),
            port: Some(1883),
            username: None,
            password: None,
            role: Some("gui".into()),
            id: Some("any".into()),
        }
    }
}

impl From<EditableTetherSettings> for TetherSettingsInProject {
    fn from(value: EditableTetherSettings) -> Self {
        let EditableTetherSettings {
            host,
            port,
            username,
            password,
            role,
            id,
            ..
        } = value;
        TetherSettingsInProject {
            host: Some(host),
            port: Some(port),
            username: Some(username),
            password: Some(password),
            role: Some(role),
            id: Some(id),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub widgets: Vec<WidgetEntry>,
    pub tether_settings: Option<TetherSettingsInProject>,
}

impl Project {
    pub fn load(&mut self, file_path: &str) -> Result<(), ()> {
        let text = fs::read_to_string(file_path);
        match text {
            Ok(d) => {
                info!("Found widget data file; parsing...");
                let project =
                    serde_json::from_str::<Project>(&d).expect("failed to parse project file");
                info!("... loaded {} widgets OK", project.widgets.len());
                *self = project;
                // TODO: optionally "broadcast" all values from loaded Widgets
                Ok(())
            }
            Err(e) => {
                error!("Failed to load widgets from disk: {:?}", e);
                Err(())
            }
        }
    }
}
