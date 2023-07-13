use std::fs;

use tether_agent::TetherAgentOptionsBuilder;

use egui::{Color32, RichText, Ui};
use log::*;
use tether_utils::tether_topics::{Insights, TopicOptions};

use crate::{
    gui::tether_gui_utils::EditableTetherSettings, project::TetherSettingsInProject, Model,
};

use super::tether_gui_utils::init_new_tether_agent;

pub fn standard_spacer(ui: &mut egui::Ui) {
    ui.add_space(16.);
}

pub fn common_remove_button(ui: &mut Ui) -> bool {
    ui.button("❌ Remove").clicked()
}

pub fn attempt_new_tether_connection(model: &mut Model) {
    let tether_options = TetherAgentOptionsBuilder::from(&model.editable_tether_settings);

    let tether_agent = init_new_tether_agent(&tether_options);

    match tether_agent.connect(&tether_options) {
        Ok(_) => {
            info!("Connected Tether Agent OK");
            model.editable_tether_settings.was_changed = true;
            model.insights = Insights::new(
                &TopicOptions {
                    topic: model.monitor_topic.clone(),
                },
                &tether_agent,
            );
        }
        Err(e) => {
            model.editable_tether_settings.is_editing = false;
            model.editable_tether_settings.was_changed = false;

            error!("Failed to connect Tether, {}", e);
        }
    }

    model.tether_agent = tether_agent;
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
              if model.editable_tether_settings.was_changed {
                  info!("Tether Settings were edited; saving these to project");
                  model.project.tether_settings = Some(TetherSettingsInProject::from(model.editable_tether_settings.clone()));
              };
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
              match model.project.load(&path_string) {
                  Ok(()) => {
                      info!("Loaded project file OK");
                      model.json_file = Some(path_string);
                      if let Some(tether_settings_in_project) = &model.project.tether_settings {
                          info!("Project file had custom Tether settings; attempt to apply and connect...");

                          model.editable_tether_settings = EditableTetherSettings::from(&tether_settings_in_project.clone());
                          attempt_new_tether_connection(model);

                      }
                  }
                  Err(_) => {
                      error!("Failed to load project file");
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

    if model.editable_tether_settings.is_editing {
        ui.horizontal(|ui| {
            ui.label("IP Address");
            ui.text_edit_singleline(&mut model.editable_tether_settings.host);
        });
        ui.horizontal(|ui| {
            ui.label("Username");
            ui.text_edit_singleline(&mut model.editable_tether_settings.username);
        });
        ui.horizontal(|ui| {
            ui.label("Password");
            ui.text_edit_singleline(&mut model.editable_tether_settings.password);
        });
        if ui.button("Apply").clicked() {
            model.editable_tether_settings.is_editing = false;
            info!("Re(creating) Tether Agent with new settings...");

            attempt_new_tether_connection(model);
        }
    } else {
        ui.label(model.tether_agent.broker_uri());

        if ui.button("Edit").clicked() {
            model.editable_tether_settings.is_editing = true;
        }
    }

    if model.tether_agent.is_connected() {
        ui.label(RichText::new("Connected ☑").color(Color32::GREEN));
    } else {
        ui.label(RichText::new("Not connected ✖").color(Color32::RED));
        if ui.button("Connect").clicked() {
            attempt_new_tether_connection(model);
        }
    }

    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Role");
        if ui
            .text_edit_singleline(&mut model.editable_tether_settings.role)
            .changed()
        {
            model
                .tether_agent
                .set_role(&model.editable_tether_settings.role);
            // model.prepare_next_entry();
        }
    });
    ui.horizontal(|ui| {
        ui.label("ID or Group");
        if ui
            .text_edit_singleline(&mut model.editable_tether_settings.id)
            .changed()
        {
            model
                .tether_agent
                .set_id(&model.editable_tether_settings.id);
            // model.prepare_next_entry();
        }
    });
}
