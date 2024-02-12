use crate::{ui::functions::integer_edit_field_u64, HitSplit};

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
                integer_edit_field_u64(ui, &mut app.config.autosave_interval, 30.0);
                ui.label("seconds");
            });
        }
        ui.horizontal(|ui| {
            ui.label("Next split as reset on last split: ");
            ui.checkbox(&mut app.config.next_split_as_reset, "");
        });

        ui.separator();
        ui.heading("Keybindings");
        ui.label("Work in progress. These are the default keys:");
        ui.horizontal(|ui| {
            ui.label("Previous split: ");
            ui.label("NUMPAD8");
        });
        ui.horizontal(|ui| {
            ui.label("Next split: ");
            ui.label("NUMPAD2");
        });
        ui.horizontal(|ui| {
            ui.label("Add hit: ");
            ui.label("NUMPAD7");
        });
        ui.horizontal(|ui| {
            ui.label("Substract hit: ");
            ui.label("NUMPAD9");
        });
        ui.horizontal(|ui| {
            ui.label("Reset: ");
            ui.label("NUMPAD5");
        });
        ui.horizontal(|ui| {
            ui.label("Set current table as PB: ");
            ui.label("NUMPAD3");
        });

        if ui.button("Save config").clicked() {
            app.config.save();
            if let Some(category) = app.loaded_category.clone() {
                category.save();
            }
        }
    });
}
