use crate::{
    settings::shortcut::ShortcutAction,
    ui::functions::{integer_edit_field_u64, shortcut_button},
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
                integer_edit_field_u64(ui, &mut app.config.autosave_interval, 30.0);
                ui.label("seconds");
            });
        }
        ui.horizontal(|ui| {
            ui.label("Next split as reset on last split: ");
            ui.checkbox(&mut app.config.next_split_as_reset, "");
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

        if ui.button("Save config").clicked() {
            app.config.save();
            if let Some(category) = app.loaded_category.clone() {
                category.save();
            }
        }
    });
}
