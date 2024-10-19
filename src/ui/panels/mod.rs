use egui::{Color32, Context};

use crate::HitSplit;

use super::{functions::image_button, VERSION};

pub mod list;
pub mod settings;

#[derive(Clone)]
pub enum Pages {
    List,
    Settings,
}

const WIDTH: f32 = 64.0;
const HEIGHT: f32 = 64.0;
const ROUNDING: f32 = 10.0;

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

            if ui
                .add(image_button(list_button, WIDTH, HEIGHT, ROUNDING))
                .clicked()
            {
                app.open_page = Pages::List;
            } else if ui
                .add(image_button(config_button, WIDTH, HEIGHT, ROUNDING))
                .clicked()
            {
                app.open_page = Pages::Settings;
            }
            egui::TopBottomPanel::bottom("version_panel")
                .exact_height(20.)
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.colored_label(egui::Color32::from_rgb(240, 107, 12), VERSION);
                });
        });
}

pub fn bottom_panel(app: &mut HitSplit, ctx: &Context) {
    egui::TopBottomPanel::bottom("error_panel")
        .exact_height(20.0)
        .resizable(false)
        .show(ctx, |ui| {
            if let crate::Error::Error(e) = &app.error {
                ui.colored_label(Color32::from_rgb(250, 8, 8), e);
            }
        });
}
