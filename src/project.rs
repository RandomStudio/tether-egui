use std::fs;

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::widgets::WidgetEntry;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TetherSettings {
    pub host: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for TetherSettings {
    fn default() -> Self {
        TetherSettings {
            host: "127.0.0.1".into(),
            username: None,
            password: None,
        }
    }
}

pub struct EditableTetherSettings {
    pub is_editing: bool,
    pub was_changed: bool,
    pub host: String,
    pub username: String,
    pub password: String,
}

impl Default for EditableTetherSettings {
    fn default() -> Self {
        EditableTetherSettings {
            is_editing: false,
            was_changed: false,
            host: "127.0.0.1".into(),
            username: "".into(),
            password: "".into(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub widgets: Vec<WidgetEntry>,
    pub tether_settings: Option<TetherSettings>,
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
