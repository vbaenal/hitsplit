use std::cmp::min;

use egui::{Color32, Context, Sense};

use crate::{settings::columns::Column, HitSplit};

pub fn counter(app: &mut HitSplit, ctx: &Context) {
    let bg = &app.config.background_color;
    let fill_color = if cfg!(target_os = "windows") {
        Color32::from_rgb(bg[0], bg[1], bg[2])
    } else {
        Color32::from_rgba_premultiplied(bg[0], bg[1], bg[2], app.config.background_transparency)
    };
    let counter_clicked = egui::CentralPanel::default()
        .frame(egui::Frame {
            fill: fill_color,
            ..Default::default()
        })
        .show(ctx, |ui| {
            let tc = &app.config.text_color_default;
            let color = Color32::from_rgb(tc[0], tc[1], tc[2]);

            let style = ui.style_mut();
            style
                .text_styles
                .get_mut(&egui::TextStyle::Body)
                .unwrap()
                .size = app.config.font_size;
            ui.vertical_centered(|ui| {
                if let Some(game) = &app.loaded_game {
                    if let Some(img) = &game.icon_path {
                        let path = img.as_path().to_str().unwrap();
                        ui.add(
                            egui::Image::new(format!("file://{path}"))
                                .max_height(app.config.game_image_height),
                        );
                    } else {
                        ui.colored_label(color, game.name.clone());
                    }
                }
            });
            ui.vertical_centered(|ui| {
                if let Some(category) = &app.loaded_category {
                    if let Some(img) = &category.icon_path {
                        let path = img.as_path().to_str().unwrap();
                        ui.add(
                            egui::Image::new(format!("file://{path}"))
                                .max_height(app.config.category_image_height),
                        );
                    } else {
                        ui.colored_label(color, category.name.clone());
                    }
                }
            });
            ui.vertical_centered(|ui| {
                let available_height = ui.available_height();
                let mut table = egui_extras::TableBuilder::new(ui)
                    .striped(false)
                    .cell_layout(egui::Layout::centered_and_justified(
                        egui::Direction::LeftToRight,
                    ))
                    .resizable(true)
                    .striped(false)
                    .min_scrolled_height(200.0)
                    .max_scroll_height(available_height);

                for column in app.config.columns.iter() {
                    table = table.column(if *column == Column::Icon {
                        egui_extras::Column::exact(app.config.font_size)
                    } else {
                        egui_extras::Column::auto()
                    })
                }

                let binding = Vec::new();
                let splits = match &app.loaded_category {
                    Some(category) => &category.splits,
                    None => &binding,
                };

                let first_split: usize = app.selected_split
                    - min(app.config.num_splits_counter >> 1, app.selected_split);
                let last_split: usize =
                    min(first_split + app.config.num_splits_counter, splits.len());
                let first_split = min(
                    first_split,
                    last_split
                        .checked_sub(app.config.num_splits_counter)
                        .unwrap_or_default(),
                );
                table
                    .header(app.config.font_size + 5.0, |mut header| {
                        for column in app.config.columns.iter() {
                            column.header(app, &mut header);
                        }
                    })
                    .body(|mut body| {
                        splits
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| {
                                !app.config.limit_splits_shown
                                    || (i >= &first_split && i < &last_split)
                            })
                            .for_each(|(i, split)| {
                                let mut label_color = color;
                                if i <= app.selected_split {
                                    let color_array: [u8; 3];
                                    if split.hits == 0 {
                                        color_array = app.config.text_color_nohit;
                                    } else if split.hits < split.pb {
                                        color_array = app.config.text_color_better;
                                    } else {
                                        color_array = app.config.text_color_worse;
                                    }
                                    label_color = Color32::from_rgb(
                                        color_array[0],
                                        color_array[1],
                                        color_array[2],
                                    );
                                }
                                body.row(app.config.font_size + 5.0, |mut row| {
                                    for column in app.config.columns.iter() {
                                        column.body(
                                            app,
                                            i,
                                            split,
                                            label_color,
                                            &app.config.chrono_format,
                                            &mut row,
                                        );
                                    }
                                });
                            });
                        body.row(app.config.font_size + 5.0, |mut row| {
                            for column in app.config.columns.iter() {
                                column.total(app, color, &mut row);
                            }
                        });
                    });
            });
        })
        .response
        .interact(Sense::click())
        .secondary_clicked();
    if counter_clicked {
        app.show_config = true;
    };
}
