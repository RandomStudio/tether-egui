use std::fs;

use egui::{Color32, RichText, Ui};
use log::*;
use tether_agent::TetherAgentOptionsBuilder;

use crate::{project::try_load, Model};

use super::tether_gui_utils::{unconnected_tether_agent, EditableTetherSettings};

pub fn standard_spacer(ui: &mut egui::Ui) {
    ui.add_space(16.);
}

pub fn common_remove_button(ui: &mut Ui) -> bool {
    ui.button("❌ Remove").clicked()
}

pub fn general_agent_area(ui: &mut Ui, model: &mut Model) {
    ui.heading("Load/Save Project");
    if let Some(json_path) = &model.json_file {
        ui.small(json_path);
    } else {
        ui.small("(No JSON file loaded)");
    }
    ui.horizontal(|ui| {
      if ui.button("Save").clicked() {
          if let Some(path) = rfd::FileDialog::new()
              .add_filter("text", &["json"])
              .save_file()
          {
            //   if model.project.tether_settings.was_changed {
            //       info!("Tether Settings were edited; copying these to project");
            //       model.project.tether_settings = Some(model.editable_tether_settings.clone());
            //   };
              let path_string = path.display().to_string();
              let text = serde_json::to_string_pretty(&model.project)
                  .expect("failed to serialise widget data");
              match fs::write(path_string, text) {
                  Ok(()) => {
                      info!("Saved OK");
                  }
                  Err(e) => {
                      error!("Error saving to disk: {:?}", e);
                  }
              }
          }
      }
      if ui.button("Load").clicked() {
          if let Some(path) = rfd::FileDialog::new()
              .add_filter("text", &["json"])
              .pick_file()
          {
              let path_string = path.display().to_string();
              let (project, loaded) = try_load(&path_string);
              if loaded {
                  info!("Loaded project file OK");
                  model.json_file = Some(path_string);
                  if let Some(tether_settings_in_project) =project.tether_settings {
                      info!("Project file had custom Tether settings; attempt to apply and connect...");

                    //   model.editable_tether_settings = tether_settings_in_project.clone();
                    //   attempt_new_tether_connection(model);
                    model.tether_agent = unconnected_tether_agent(&TetherAgentOptionsBuilder::from(tether_settings_in_project))
                  }

              }
          }
      }
      if ui.button("Clear").clicked() {
          model.project.widgets.clear();
          model.json_file = None;
      }
  });

    standard_spacer(ui);
    ui.separator();
    ui.heading("Agent");

    if let Some(tether_settings) = &mut model.project.tether_settings {
        ui.horizontal(|ui| {
            ui.label("IP Address");
            ui.text_edit_singleline(&mut tether_settings.host);
        });
        ui.horizontal(|ui| {
            ui.label("Username");
            ui.text_edit_singleline(&mut tether_settings.username);
        });
        ui.horizontal(|ui| {
            ui.label("Password");
            ui.text_edit_singleline(&mut tether_settings.password);
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Role");
            if ui.text_edit_singleline(&mut tether_settings.role).changed() {
                model.tether_agent.set_role(&tether_settings.role);
            }
        });
        ui.horizontal(|ui| {
            ui.label("ID or Group");
            if ui.text_edit_singleline(&mut tether_settings.id).changed() {
                model.tether_agent.set_id(&tether_settings.id);
            }
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                model.attempt_new_tether_connection();
            }
            if ui.button("Reset").clicked() {
                model.project.tether_settings = None;
                model.attempt_new_tether_connection();
            }
        });
    } else {
        ui.label(model.tether_agent.broker_uri());

        if ui.button("Edit").clicked() {
            model.project.tether_settings = Some(EditableTetherSettings::default());
        }
    }

    if model.tether_agent.is_connected() {
        ui.label(RichText::new("Connected ☑").color(Color32::GREEN));
    } else {
        ui.label(RichText::new("Not connected ✖").color(Color32::RED));
        if ui.button("Connect").clicked() {
            model.attempt_new_tether_connection();
        }
    }
}
