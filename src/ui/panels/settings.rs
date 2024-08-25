use egui::Slider;

use crate::{
    run::chrono::ChronometerFormat,
    settings::shortcut::ShortcutAction,
    ui::functions::{numeric_edit_field_u64, shortcut_button},
    HitSplit,
};

pub fn configuration(app: &mut HitSplit, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Configuration");
        ui.horizontal(|ui| {
            ui.label("Visual mode: ");
            egui::widgets::global_dark_light_mode_buttons(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Autosave: ");
            ui.checkbox(&mut app.config.autosave, "");
        });
        if app.config.autosave {
            ui.horizontal(|ui| {
                ui.label("Autosave interval: ");
                numeric_edit_field_u64(ui, &mut app.config.autosave_interval, 30.0);
                ui.label("seconds");
            });
        }
        ui.horizontal(|ui| {
            ui.label("Next split as reset on last split: ");
            ui.checkbox(&mut app.config.next_split_as_reset, "");
        });
        ui.horizontal(|ui| {
            ui.label("Text font size: ");
            ui.add(Slider::new(&mut app.config.font_size, 1.0..=100.0));
        });
        ui.horizontal(|ui| {
            ui.label("Show a limited number of splits: ");
            ui.checkbox(&mut app.config.limit_splits_shown, "");
        });
        if app.config.limit_splits_shown {
            ui.horizontal(|ui| {
                ui.label("Number of splits shown: ");
                ui.add(Slider::new(&mut app.config.num_splits_counter, 1..=25));
            });
        }
        ui.horizontal(|ui| {
            ui.label("Chronometer format");
            let chrono_format = &app.config.chrono_format.clone();
            let fun = |ui: &mut egui::Ui| {
                ui.selectable_value(
                    &mut app.config.chrono_format,
                    ChronometerFormat::HHMM,
                    "H:MM",
                );
                ui.selectable_value(
                    &mut app.config.chrono_format,
                    ChronometerFormat::HHMMSS,
                    "H:MM:SS",
                );
                ui.selectable_value(
                    &mut app.config.chrono_format,
                    ChronometerFormat::HHMMSSX,
                    "H:MM:SS.cs",
                );
                ui.selectable_value(
                    &mut app.config.chrono_format,
                    ChronometerFormat::MMSS,
                    "MM:SS",
                );
                ui.selectable_value(
                    &mut app.config.chrono_format,
                    ChronometerFormat::MMSSX,
                    "MM:SS.cs",
                );
            };
            if egui::ComboBox::from_id_source("chrono_format")
                .selected_text(chrono_format.text())
                .show_ui(ui, fun)
                .response
                .changed()
            {
                app.chrono.set_format(chrono_format);
            }
        });
        ui.separator();
        ui.heading("Shortcuts");
        ui.horizontal(|ui| {
            ui.label("Previous split: ");
            shortcut_button(app, ui, &ShortcutAction::PrevSplit);
        });
        ui.horizontal(|ui| {
            ui.label("Next split: ");
            shortcut_button(app, ui, &ShortcutAction::NextSplit);
        });
        ui.horizontal(|ui| {
            ui.label("Add hit: ");
            shortcut_button(app, ui, &ShortcutAction::AddHit);
        });
        ui.horizontal(|ui| {
            ui.label("Substract hit: ");
            shortcut_button(app, ui, &ShortcutAction::SubHit);
        });
        ui.horizontal(|ui| {
            ui.label("Reset: ");
            shortcut_button(app, ui, &ShortcutAction::Reset);
        });
        ui.horizontal(|ui| {
            ui.label("Set current run as PB: ");
            shortcut_button(app, ui, &ShortcutAction::SetPb);
        });
        ui.horizontal(|ui| {
            ui.label("Start chrono: ");
            shortcut_button(app, ui, &ShortcutAction::StartChrono);
        });
        ui.horizontal(|ui| {
            ui.label("Pause chrono: ");
            shortcut_button(app, ui, &ShortcutAction::PauseChrono);
        });

        if ui.button("Save config").clicked() {
            app.config.save();
            app.shortcut.as_ref().unwrap().save();
            if let Some(category) = app.loaded_category.clone() {
                category.save();
            }
        }
    });
}
