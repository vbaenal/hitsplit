use std::cmp::min;

use egui::{Color32, Context};

use crate::HitSplit;

pub fn counter(app: &mut HitSplit, ctx: &Context) {
    let app_cl = app.clone();
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("hitsplit_counter"),
        egui::ViewportBuilder::default()
            .with_title("HitSplit Counter")
            .with_resizable(true)
            .with_inner_size(app.config.counter_size),
        move |ctx, _class| {
            app.config.counter_size = ctx.screen_rect().size();
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(app_cl.loaded_game.unwrap().name);
                });
                if app_cl.loaded_category.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.label(app_cl.loaded_category.as_ref().unwrap().name.clone());
                    });
                }

                ui.vertical(|ui| {
                    let table = egui_extras::TableBuilder::new(ui)
                        .striped(false)
                        .cell_layout(egui::Layout::centered_and_justified(
                            egui::Direction::LeftToRight,
                        ))
                        .resizable(true)
                        .striped(false)
                        .column(egui_extras::Column::exact(app.config.font_size))
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .min_scrolled_height(200.0);
                    let mut color = Color32::from_rgb(250, 250, 250);
                    if !app_cl.config.dark_mode {
                        color = Color32::from_rgb(8, 8, 8)
                    }
                    let splits = &app_cl.loaded_category.as_ref().unwrap().splits;
                    let first_split: usize = app_cl.selected_split
                        - min(app_cl.config.num_splits_counter >> 1, app_cl.selected_split);
                    let last_split: usize =
                        min(first_split + app_cl.config.num_splits_counter, splits.len());
                    let first_split = min(
                        first_split,
                        last_split
                            .checked_sub(app_cl.config.num_splits_counter)
                            .unwrap_or_default(),
                    );
                    table
                        .header(app.config.font_size + 5.0, |mut header| {
                            header.col(|_| {});
                            header.col(|ui| {
                                ui.strong("Name");
                            });
                            header.col(|ui| {
                                ui.strong("Hits");
                            });
                            header.col(|ui| {
                                ui.strong("Diff");
                            });
                            header.col(|ui| {
                                ui.strong("PB");
                            });
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
                                    if i <= app_cl.selected_split {
                                        if split.hits == 0 {
                                            label_color = Color32::from_rgb(8, 250, 8);
                                        } else if split.hits < split.pb {
                                            label_color = Color32::from_rgb(250, 250, 8);
                                        } else {
                                            label_color = Color32::from_rgb(250, 8, 8);
                                        }
                                    }
                                    body.row(app.config.font_size + 5.0, |mut row| {
                                        let mut name = split.name.clone();
                                        if i == app_cl.selected_split {
                                            name = format!("> {}", name);
                                        }
                                        row.col(|ui| {
                                            if let Some(p) = &split.icon_path {
                                                let path = p.as_path().to_str().unwrap();
                                                ui.add(
                                                    egui::Image::new(format!("file://{path}"))
                                                        .max_height(app.config.font_size),
                                                );
                                            }
                                        });
                                        row.col(|ui| {
                                            ui.colored_label(label_color, name);
                                        });
                                        row.col(|ui| {
                                            ui.colored_label(label_color, split.hits.to_string());
                                        });
                                        row.col(|ui| {
                                            ui.colored_label(
                                                label_color,
                                                (i32::from(split.hits) - i32::from(split.pb))
                                                    .to_string(),
                                            );
                                        });
                                        row.col(|ui| {
                                            ui.colored_label(label_color, split.pb.to_string());
                                        });
                                    });
                                });
                            body.row(app.config.font_size + 5.0, |mut row| {
                                row.col(|_| {});
                                row.col(|ui| {
                                    ui.colored_label(color, "Total: ");
                                });
                                row.col(|ui| {
                                    let hits = app_cl
                                        .loaded_category
                                        .as_ref()
                                        .unwrap()
                                        .splits
                                        .iter()
                                        .map(|split| split.hits);
                                    ui.colored_label(color, hits.sum::<u16>().to_string());
                                });
                                row.col(|ui| {
                                    let diffs =
                                        app_cl.loaded_category.as_ref().unwrap().splits.iter().map(
                                            |split| i32::from(split.hits) - i32::from(split.pb),
                                        );
                                    ui.colored_label(color, diffs.sum::<i32>().to_string());
                                });
                                row.col(|ui| {
                                    let pbs = app_cl
                                        .loaded_category
                                        .as_ref()
                                        .unwrap()
                                        .splits
                                        .iter()
                                        .map(|split| split.pb);
                                    ui.colored_label(color, pbs.sum::<u16>().to_string());
                                });
                            });
                        });
                });
            });
            if ctx.input(|i| i.raw.viewport().close_requested()) {
                app.show_hit_counter = false;
                ctx.request_repaint();
            }
        },
    );
}
