use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use crate::ui::{
    common_editable_values, common_in_use_heading, common_save_button, common_send,
    common_send_button,
};

use super::{Common, CustomWidget, View};

pub type ColourRGBA8 = [u8; 4];

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColourWidget<ColourRGBA8> {
    common: Common,
    value: ColourRGBA8,
}

impl ColourWidget<ColourRGBA8> {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        rgba: ColourRGBA8,
        agent: &TetherAgent,
    ) -> Self {
        ColourWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: rgba,
        }
    }
}

impl CustomWidget<ColourRGBA8> for ColourWidget<ColourRGBA8> {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &ColourRGBA8 {
        &self.value
    }

    fn value_mut(&mut self) -> &mut ColourRGBA8 {
        &mut self.value
    }
}

impl View for ColourWidget<ColourRGBA8> {
    fn render_in_use(&mut self, ctx: &egui::Context, index: usize, tether_agent: &TetherAgent) {
        egui::Window::new(&self.common.name)
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_in_use_heading(ui, self);

                if ui
                    .color_edit_button_srgba_unmultiplied(self.value_mut())
                    .changed()
                    || common_send_button(ui, self).clicked()
                {
                    common_send(self, tether_agent);
                }
            });
    }

    fn render_editing(&mut self, ctx: &egui::Context, index: usize, tether_agent: &TetherAgent) {
        egui::Window::new(&self.common.name)
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_editable_values(ui, self, tether_agent);
                common_save_button(ui, self);
            });
    }
}
