use std::{fs, net::IpAddr};

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::widgets::WidgetEntry;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TetherSettings {
    pub tether_host: IpAddr,
}

#[derive(Serialize, Deserialize)]
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

impl Default for Project {
    fn default() -> Self {
        Project {
            widgets: Vec::new(),
            tether_settings: None,
        }
    }
}
