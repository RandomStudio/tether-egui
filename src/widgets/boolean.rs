use egui::Ui;
use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use crate::ui::{
    common_editable_values, common_in_use_heading, common_save_button, common_widget_values,
    ENTRY_GRID_WIDTH,
};

use super::{Common, CustomWidget, View};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoolWidget {
    common: Common,
    value: bool,
}

impl BoolWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        init_state: bool,
        agent: &TetherAgent,
    ) -> Self {
        BoolWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: init_state,
        }
    }
}

impl CustomWidget<bool> for BoolWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &bool {
        &self.value
    }

    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl View for BoolWidget {
    fn render_in_use(&mut self, ctx: &egui::Context, index: usize) {
        egui::Window::new("Bool")
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_in_use_heading(ui, self);

                let checked = *self.value();
                let checkbox = ui.checkbox(
                    self.value_mut(),
                    format!("State: {}", {
                        if checked {
                            "TRUE"
                        } else {
                            "FALSE "
                        }
                    }),
                );

                ui.button("Send");
            });
    }

    fn render_editing(&mut self, ctx: &egui::Context, index: usize) {
        egui::Window::new("Boolean")
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_editable_values(ui, self);
                common_save_button(ui, self);
            });
    }
}
