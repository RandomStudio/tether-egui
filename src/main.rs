#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use tweaks::{NumberTweak, Tweak};

mod tweaks;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 960.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<Model>::default()),
    )
}

struct Model {
    next_name: String,
    next_description: String,
    tweaks: Vec<Box<dyn Tweak>>,
}

// impl Model {
//     pub fn add_number_entry(name: String, description: String) -> NumberTweak {
//         NumberTweak {
//             name,
//             description,
//             value: 0.,
//             range: None
//         }
//     }
// }

impl Default for Model {
    fn default() -> Self {
        Self {
            next_name: String::from("Entry"),
            next_description: String::from(""),
            tweaks: Vec::new(),
        }
    }
}

impl eframe::App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Custom UI")
            .min_width(512.)
            .show(ctx, |ui| {
                ui.heading("Entries");
                for entry in &self.tweaks {
                    ui.label(entry.name());
                    ui.small(entry.description());
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("UI builder");

            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.next_name);
            });
            ui.horizontal(|ui| {
                ui.label("Description");
                ui.text_edit_singleline(&mut self.next_description);
            });
            if ui.button("Add number value").clicked() {
                self.tweaks.push(Box::new(NumberTweak {
                    name: self.next_name.clone(),
                    description: self.next_description.clone(),
                    range: None,
                    value: 0.,
                }));
            }
        });
    }
}
