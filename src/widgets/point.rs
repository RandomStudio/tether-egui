use egui::{
    plot::{Plot, PlotPoint},
    Ui,
};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use tether_agent::TetherAgent;

use crate::{
    gui::widget_view::{
        common_editable_values, common_in_use_heading, common_save_button, common_send_button,
    },
    midi_mapping::MidiMapping,
};

use super::{Common, CustomWidget, View};

type Point2D = [f64; 2];

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point2DWidget {
    common: Common,
    value: Point2D,
}

impl Point2DWidget {
    pub fn new(
        widget_name: &str,
        description: Option<&str>,
        plug_name: &str,
        custom_topic: Option<&str>,
        agent: &mut TetherAgent,
    ) -> Self {
        Point2DWidget {
            common: Common::new(widget_name, description, plug_name, custom_topic, agent),
            value: [0., 0.],
        }
    }
}

impl CustomWidget<Point2D> for Point2DWidget {
    fn common(&self) -> &Common {
        &self.common
    }
    fn common_mut(&mut self) -> &mut Common {
        &mut self.common
    }
    fn value(&self) -> &Point2D {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Point2D {
        &mut self.value
    }
}

const PLOT_SIZE: f32 = 200.0;

impl View for Point2DWidget {
    fn render_editing(&mut self, ui: &mut Ui, tether_agent: &mut TetherAgent) {
        common_editable_values(ui, self, tether_agent);
        common_save_button(ui, self, tether_agent);
    }

    fn render_in_use(&mut self, ui: &mut Ui, tether_agent: &TetherAgent) {
        common_in_use_heading(ui, self);

        if let Some(midi) = &self.common().midi_mapping {
            match midi {
                MidiMapping::Learning => {}
                MidiMapping::Set(mapping) => {
                    ui.label(format!(
                        "MIDI mapped: send on ch {} note {}",
                        mapping.channel, mapping.controller_or_note
                    ));
                }
            }
        }

        let plot = Plot::new("tracking_plot")
            .width(PLOT_SIZE)
            .height(PLOT_SIZE)
            .data_aspect(1.0)
            .show(ui, |plot_ui| {
                (
                    plot_ui.screen_from_plot(PlotPoint::new(0.0, 0.0)),
                    plot_ui.pointer_coordinate(),
                    plot_ui.pointer_coordinate_drag_delta(),
                    plot_ui.plot_bounds(),
                    plot_ui.plot_hovered(),
                )
            });
        ui.collapsing("Instructions", |ui| {
            ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
            ui.label("Box zooming: Right click to zoom in and zoom out using a selection.");
            if cfg!(target_arch = "wasm32") {
                ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
            } else if cfg!(target_os = "macos") {
                ui.label("Zoom with ctrl / ⌘ + scroll.");
            } else {
                ui.label("Zoom with ctrl + scroll.");
            }
            ui.label("Reset view with double-click.");
        });

        let (_screen_pos, pointer_coordinate, _pointer_coordinate_drag_delta, _bounds, hovered) =
            plot.inner;

        if common_send_button(ui, self, true).clicked() || hovered && self.common().auto_send {
            if let Some(c) = pointer_coordinate {
                // println!("Pointer coordinates: {:?}", c)
                let PlotPoint { x, y } = c;
                let p = [x, y];
                match tether_agent.encode_and_publish(&self.common().plug, p) {
                    Ok(()) => debug!("Send OK"),
                    Err(_) => error!("Failed to send; connected? {}", tether_agent.is_connected()),
                }
            }
        }
    }
}
