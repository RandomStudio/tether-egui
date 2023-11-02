use std::fs;

use log::{info, warn};
use serde::{Deserialize, Serialize};

use crate::{gui::tether_gui_utils::EditableTetherSettings, widgets::WidgetEntry};

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub widgets: Vec<WidgetEntry>,
    pub tether_settings: Option<EditableTetherSettings>,
}

pub fn try_load(file_path: &str) -> (Project, bool) {
    let text = fs::read_to_string(file_path);
    match text {
        Ok(d) => {
            info!("Found widget data file; parsing...");
            let project =
                serde_json::from_str::<Project>(&d).expect("failed to parse project file");
            info!("... loaded {} widgets OK", project.widgets.len());
            // TODO: optionally "broadcast" all initial values from loaded Widgets
            (project, true)
        }
        Err(e) => {
            warn!("Failed to load widgets from disk: {:?}", e);
            (Project::default(), false)
        }
    }
}
