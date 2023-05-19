use egui::Color32;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tether_agent::TetherAgent;

use crate::ui::{
    common_editable_values, common_in_use_heading, common_save_button, common_send,
    common_send_button,
};

use super::{Common, CustomWidget, View};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericJSONWidget {
    common: Common,
    value: String,
    #[serde(skip, default = "assume_valid")]
    is_valid_json: bool,
}

fn assume_valid() -> bool {
    true
}

impl GenericJSONWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        GenericJSONWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: "{\"answer\":42}".into(),
            is_valid_json: true,
        }
    }
}

impl CustomWidget<String> for GenericJSONWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &String {
        &self.value
    }

    fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl View for GenericJSONWidget {
    fn render_editing(&mut self, ctx: &egui::Context, index: usize, tether_agent: &TetherAgent) {
        egui::Window::new(&self.common.name)
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_editable_values(ui, self, tether_agent);
                common_save_button(ui, self);
            });
    }

    fn render_in_use(&mut self, ctx: &egui::Context, index: usize, tether_agent: &TetherAgent) {
        egui::Window::new(&self.common.name)
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_in_use_heading(ui, self);

                if ui.text_edit_multiline(self.value_mut()).changed() {
                    if serde_json::from_str::<Value>(self.value()).is_err() {
                        self.is_valid_json = false;
                    } else {
                        self.is_valid_json = true;
                    }
                }
                if self.is_valid_json {
                    ui.colored_label(Color32::LIGHT_GREEN, "Valid JSON");
                } else {
                    ui.colored_label(Color32::RED, "Not valid JSON");
                }

                if common_send_button(ui, self).clicked() {
                    common_send(self, tether_agent);
                }
            });
    }
}
