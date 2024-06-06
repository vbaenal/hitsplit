use std::cmp::min;

use egui::{Color32, Context, Sense};

use crate::{
    run::{category::Category, game::Game},
    settings::columns::Column,
    HitSplit,
};

pub fn counter(app: &mut HitSplit, ctx: &Context) {
    let counter_clicked = egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.style_mut()
                .text_styles
                .get_mut(&egui::TextStyle::Body)
                .unwrap()
                .size = app.config.font_size;
            ui.vertical_centered(|ui| {
                ui.label(match &app.loaded_game {
                    Some(game) => game.name.clone(),
                    None => Game::default().name,
                });
            });
            if app.loaded_category.is_some() {
                ui.vertical_centered(|ui| {
                    ui.label(match &app.loaded_category {
                        Some(category) => category.name.clone(),
                        None => Category::default().name,
                    });
                });
            }

            ui.vertical(|ui| {
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

                let mut color = Color32::from_rgb(250, 250, 250);
                if !app.config.dark_mode {
                    color = Color32::from_rgb(8, 8, 8)
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
                                    if split.hits == 0 {
                                        label_color = Color32::from_rgb(8, 250, 8);
                                    } else if split.hits < split.pb {
                                        label_color = Color32::from_rgb(250, 250, 8);
                                    } else {
                                        label_color = Color32::from_rgb(250, 8, 8);
                                    }
                                }
                                body.row(app.config.font_size + 5.0, |mut row| {
                                    for column in app.config.columns.iter() {
                                        column.body(app, i, split, label_color, &mut row);
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
