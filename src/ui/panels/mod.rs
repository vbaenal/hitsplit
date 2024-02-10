use egui::Context;

use crate::HitSplit;

use super::functions::image_button;

pub mod list;
pub mod settings;

#[derive(Clone)]
pub enum Pages {
    List,
    Settings,
}

pub fn left_panel(app: &mut HitSplit, ctx: &Context) {
    egui::SidePanel::left("left_panel")
        .exact_width(72.)
        .resizable(false)
        .show(ctx, |ui| {
            let list_button;
            let config_button;
            if app.config.dark_mode {
                list_button = egui::include_image!("../../assets/dark_mode/list.svg");
                config_button = egui::include_image!("../../assets/dark_mode/config.svg");
            } else {
                list_button = egui::include_image!("../../assets/light_mode/list.svg");
                config_button = egui::include_image!("../../assets/light_mode/config.svg");
            }

            if ui.add(image_button(list_button)).clicked() {
                app.open_page = Pages::List;
            } else if ui.add(image_button(config_button)).clicked() {
                app.open_page = Pages::Settings;
            }
        });
}
