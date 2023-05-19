use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use crate::ui::{common_editable_values, common_in_use_heading, common_widget_values};

use super::{Common, CustomWidget, View};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyWidget {
    common: Common,
    value: (),
}

impl EmptyWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &TetherAgent,
    ) -> Self {
        EmptyWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: (),
        }
    }
}

impl CustomWidget<()> for EmptyWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &() {
        &self.value
    }

    fn value_mut(&mut self) -> &mut () {
        &mut self.value
    }
}

impl View for EmptyWidget {
    fn render_in_use(&mut self, ctx: &egui::Context, index: usize) {
        egui::Window::new("Empty Message")
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_in_use_heading(ui, self);

                ui.button("Send");
            });
    }
    fn render_editing(&mut self, ctx: &egui::Context, index: usize) {
        egui::Window::new("Empty Message")
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_editable_values(ui, self);
                if ui.button("Save").clicked() {
                    self.common_mut().set_edit_mode(false);
                }
            });
    }
}
