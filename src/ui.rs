use crate::{
    widgets::{BoolWidget, ColourWidget, NumberWidget},
    Model, WidgetEntry,
};

pub fn standard_spacer(ui: &mut egui::Ui) {
    ui.add_space(16.);
}

pub fn available_widgets(ctx: &egui::Context, model: &mut Model) {
    egui::Window::new("Floating-Point Number").show(ctx, |ui| {
        model.common_widget_values(ui);

        standard_spacer(ui);

        ui.label("Range");
        ui.add(
            egui::Slider::new(&mut model.next_range.0, i16::MIN as f32..=i16::MAX as f32)
                .text("min"),
        );
        ui.add(
            egui::Slider::new(&mut model.next_range.1, i16::MIN as f32..=i16::MAX as f32)
                .text("max"),
        );
        if ui.small_button("Reset").clicked() {
            model.next_range = (0., 1.0);
        }
        ui.separator();

        if ui.button("✚ Add").clicked() {
            model
                .widgets
                .push(WidgetEntry::FloatNumber(NumberWidget::new(
                    &model.next_widget.name,
                    {
                        if model.next_widget.description.is_empty() {
                            None
                        } else {
                            Some(&model.next_widget.description)
                        }
                    },
                    &model.next_widget.plug.name,
                    {
                        if model.use_custom_topic {
                            Some(&model.next_topic)
                        } else {
                            None
                        }
                    },
                    0.,
                    model.next_range.0.into()..=model.next_range.1.into(),
                    &model.tether_agent,
                )));
            model.prepare_next_entry();
        }
    });

    egui::Window::new("Whole Number").show(ctx, |ui| {
        model.common_widget_values(ui);

        standard_spacer(ui);

        ui.label("Range");
        ui.add(
            egui::Slider::new(&mut model.next_range.0, i16::MIN as f32..=i16::MAX as f32)
                .text("min"),
        );
        ui.add(
            egui::Slider::new(&mut model.next_range.1, i16::MIN as f32..=i16::MAX as f32)
                .text("max"),
        );
        if ui.small_button("Reset").clicked() {
            model.next_range = (0., 100.);
        }
        ui.separator();

        if ui.button("✚ Add").clicked() {
            let min = model.next_range.0 as i64;
            let max = model.next_range.1 as i64;
            model
                .widgets
                .push(WidgetEntry::WholeNumber(NumberWidget::new(
                    &model.next_widget.name,
                    {
                        if model.next_widget.description.is_empty() {
                            None
                        } else {
                            Some(&model.next_widget.description)
                        }
                    },
                    &model.next_widget.plug.name,
                    {
                        if model.use_custom_topic {
                            Some(&model.next_topic)
                        } else {
                            None
                        }
                    },
                    0,
                    min..=max,
                    &model.tether_agent,
                )));
            model.prepare_next_entry();
        }
    });

    egui::Window::new("Colour").show(ctx, |ui| {
        model.common_widget_values(ui);
        ui.separator();
        if ui.button("✚ Add").clicked() {
            model.widgets.push(WidgetEntry::Colour(ColourWidget::new(
                model.next_widget.name.as_str(),
                {
                    if model.next_widget.description.is_empty() {
                        None
                    } else {
                        Some(&model.next_widget.description)
                    }
                },
                &model.next_widget.plug.name,
                {
                    if model.use_custom_topic {
                        Some(&model.next_topic)
                    } else {
                        None
                    }
                },
                [255, 255, 255, 255],
                &model.tether_agent,
            )));
            model.prepare_next_entry();
        }
    });

    egui::Window::new("Boolean").show(ctx, |ui| {
        model.common_widget_values(ui);
        ui.separator();
        if ui.button("✚ Add").clicked() {
            model.widgets.push(WidgetEntry::Bool(BoolWidget::new(
                model.next_widget.name.as_str(),
                {
                    if model.next_widget.description.is_empty() {
                        None
                    } else {
                        Some(&model.next_widget.description)
                    }
                },
                &model.next_widget.plug.name,
                {
                    if model.use_custom_topic {
                        Some(&model.next_topic)
                    } else {
                        None
                    }
                },
                false,
                &model.tether_agent,
            )));
            model.prepare_next_entry();
        }
    });
}
