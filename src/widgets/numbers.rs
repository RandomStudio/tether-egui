use egui::emath::Numeric;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use tether_agent::TetherAgent;

use crate::ui::{
    common_editable_values, common_in_use_heading, common_save_button, common_send,
    common_send_button,
};

use super::{Common, CustomWidget, View};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberWidget<T> {
    common: Common,
    value: T,
    range: RangeInclusive<T>,
}

impl<T: Numeric> NumberWidget<T> {
    pub fn new(
        name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        value: T,
        range: RangeInclusive<T>,
        agent: &TetherAgent,
    ) -> Self {
        NumberWidget {
            common: Common::new(name, description, plug_name, custom_topic, agent),
            value,
            range,
        }
    }

    pub fn range(&self) -> (T, T) {
        (*self.range.start(), *self.range.end())
    }
}

impl<T: Numeric + Serialize> CustomWidget<T> for NumberWidget<T> {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &T {
        &self.value
    }
    fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Numeric + Serialize> View for NumberWidget<T> {
    fn render_in_use(&mut self, ctx: &egui::Context, index: usize, tether_agent: &TetherAgent) {
        egui::Window::new(&self.common.name)
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_in_use_heading(ui, self);

                if common_send_button(ui, self).clicked() {
                    common_send(self, tether_agent);
                };
            });
    }
    fn render_editing(&mut self, ctx: &egui::Context, index: usize) {
        egui::Window::new(&self.common.name)
            .id(format!("{}", index).into())
            .show(ctx, |ui| {
                common_editable_values(ui, self);
                common_save_button(ui, self);
            });
    }
}
