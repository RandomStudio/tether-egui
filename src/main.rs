#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::Slider;
use tweaks::{ColourTweak, NumberTweak, Tweak};

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

enum TweakEntry {
    Number(NumberTweak),
    Colour(ColourTweak),
}
struct Model {
    next_name: String,
    next_description: String,
    tweaks: Vec<TweakEntry>,
    queue: Vec<QueueItem>,
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
            queue: Vec::new(),
        }
    }
}

enum QueueItem {
    Remove(usize),
}

impl eframe::App for Model {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Some(q) = self.queue.pop() {
            match q {
                QueueItem::Remove(index) => {
                    self.tweaks.remove(index);
                }
            }
        }

        egui::SidePanel::right("Custom UI")
            .min_width(512.)
            .show(ctx, |ui| {
                ui.heading("Entries");
                for (i, entry) in self.tweaks.iter_mut().enumerate() {
                    match entry {
                        TweakEntry::Number(e) => {
                            ui.label(&format!("Number: {}", e.name()));
                            let (min, max) = e.range();
                            ui.add(Slider::new(e.value_mut(), min..=max));
                        }
                        TweakEntry::Colour(e) => {
                            ui.label(&format!("Colour: {}", e.name()));
                            ui.color_edit_button_srgba_unmultiplied(e.value_mut());
                            let srgba = e.value();
                            ui.label(format!(
                                "sRGBA: {} {} {} {}",
                                srgba[0], srgba[1], srgba[2], srgba[3],
                            ));
                            ui.small(e.description());
                        }
                    }

                    if ui.button("remove").clicked() {
                        self.queue.push(QueueItem::Remove(i));
                    }

                    ui.separator();
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
            if ui.button("Add Number value").clicked() {
                self.tweaks.push(TweakEntry::Number(NumberTweak::new(
                    self.next_name.clone(),
                    self.next_description.clone(),
                    0.,
                    None,
                )));
            }
            if ui.button("Add Colour value").clicked() {
                self.tweaks.push(TweakEntry::Colour(ColourTweak::new(
                    self.next_name.clone(),
                    self.next_description.clone(),
                    (255, 255, 255, 255),
                )));
            }
        });
    }
}
