use std::sync::atomic::Ordering;

use egui::{Color32, Context};

use crate::HitSplit;

pub fn counter(app: &mut HitSplit, ctx: &Context) {
    let show_hit_counter = app.show_hit_counter.clone();
    let app_cl = app.clone();
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("hitsplit_counter"),
        egui::ViewportBuilder::default()
            .with_title("HitSplit Counter")
            .with_resizable(true)
            .with_inner_size([280.0, 600.0]),
        move |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(app_cl.loaded_game.unwrap().name);
                });
                if app_cl.loaded_category.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.label(app_cl.loaded_category.unwrap().name);
                    });
                }

                ui.vertical(|ui| {
                    let table = egui_extras::TableBuilder::new(ui)
                        .striped(false)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::LEFT))
                        .resizable(true)
                        .striped(false)
                        .column(egui_extras::Column::auto().at_least(150.0))
                        .column(egui_extras::Column::auto().at_most(22.0))
                        .column(egui_extras::Column::auto().at_most(22.0))
                        .column(egui_extras::Column::auto().at_most(22.0))
                        .min_scrolled_height(200.0);
                    let mut color = Color32::from_rgb(250, 250, 250);
                    let style: egui::Style = (*ctx.style()).clone();
                    if !style.visuals.dark_mode {
                        color = Color32::from_rgb(8, 8, 8)
                    }
                    table
                        .header(20.0, |mut header| {
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
                            app_cl
                                .loaded_splits
                                .iter()
                                .enumerate()
                                .for_each(|(i, split)| {
                                    let mut label_color = color.clone();
                                    if i <= app_cl.selected_split as usize {
                                        if split.hits == 0 {
                                            label_color = Color32::from_rgb(8, 250, 8);
                                        } else if split.hits < split.pb {
                                            label_color = Color32::from_rgb(250, 250, 8);
                                        } else {
                                            label_color = Color32::from_rgb(250, 8, 8);
                                        }
                                    }
                                    body.row(18., |mut row| {
                                        let mut name = split.name.clone();
                                        if i == app_cl.selected_split as usize {
                                            name = format!("> {}", name);
                                        }
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
                            body.row(24., |mut row| {
                                row.col(|ui| {
                                    ui.colored_label(color, "Total: ");
                                });
                                row.col(|ui| {
                                    let hits = app_cl.loaded_splits.iter().map(|split| split.hits);
                                    ui.colored_label(color, hits.sum::<u16>().to_string());
                                });
                                row.col(|ui| {
                                    let diffs = app_cl
                                        .loaded_splits
                                        .iter()
                                        .map(|split| i32::from(split.hits) - i32::from(split.pb));
                                    ui.colored_label(color, diffs.sum::<i32>().to_string());
                                });
                                row.col(|ui| {
                                    let pbs = app_cl.loaded_splits.iter().map(|split| split.pb);
                                    ui.colored_label(color, pbs.sum::<u16>().to_string());
                                });
                            });
                        });
                });
            });
            if ctx.input(|i| i.raw.viewport().close_requested()) {
                // Tell parent to close us.
                show_hit_counter.store(false, Ordering::Relaxed);
                ctx.request_repaint(); // make sure there is a next frame
            }
        },
    );
}
