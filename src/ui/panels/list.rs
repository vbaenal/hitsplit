use egui::{Color32, Context};
use uuid::Uuid;

use crate::{
    run::{
        category::{Category, SmallCategory},
        game::{Game, SmallGame},
        split::Split,
    },
    ui::functions::integer_edit_field_u16,
    HitSplit,
};

fn add_game(app: &mut HitSplit, ctx: &Context) {
    egui::Window::new("Add game")
        .fixed_pos([50., 50.])
        .resizable(false)
        .open(&mut app.add_game_open.clone())
        .title_bar(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| ui.label("Enter the name of the game"));
            if app.add_game_empty {
                ui.colored_label(Color32::from_rgb(250, 8, 8), "You must enter a game name!");
            }
            ui.horizontal(|ui| ui.add(egui::TextEdit::singleline(&mut app.add_game_name)));
            ui.horizontal(|ui| {
                if ui.small_button("Add").clicked() {
                    if app.add_game_name.eq("") {
                        app.add_game_empty = true;
                    } else {
                        let uuid: String = Uuid::new_v4().to_string();
                        app.config.game_list.push(SmallGame {
                            uuid: uuid.clone(),
                            name: app.add_game_name.clone(),
                        });
                        let game = Game::new(uuid, app.add_game_name.clone());
                        game.save();
                        app.add_game_name = "".to_string();
                        app.add_game_empty = false;
                        app.add_game_open = false;
                        app.config.save();
                        app.num_splits_category = 0;
                        app.loaded_category = None;
                        app.loaded_game = Some(game);
                    }
                }
                if ui.small_button("Cancel").clicked() {
                    app.add_game_name = "".to_string();
                    app.add_game_empty = false;
                    app.add_game_open = false;
                }
            })
        });
}

fn add_category(app: &mut HitSplit, ctx: &Context) {
    egui::Window::new("Add category")
        .fixed_pos([50., 50.])
        .resizable(false)
        .open(&mut app.add_category_open.clone())
        .title_bar(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| ui.label("Enter the name of the category"));
            if app.add_category_empty {
                ui.colored_label(
                    Color32::from_rgb(250, 8, 8),
                    "You must enter a category name!",
                );
            }
            ui.horizontal(|ui| ui.add(egui::TextEdit::singleline(&mut app.add_category_name)));
            ui.horizontal(|ui| {
                if ui.small_button("Add").clicked() {
                    if app.add_category_name.eq("") {
                        app.add_category_empty = true;
                    } else {
                        let mut game: Game = app.loaded_game.as_ref().unwrap().clone();
                        let uuid: String = Uuid::new_v4().to_string();
                        app.add_category_open = false;
                        game.categories.push(SmallCategory {
                            uuid: uuid.clone(),
                            name: app.add_category_name.clone(),
                        });
                        let category = Category::new(uuid.clone(), app.add_category_name.clone());
                        app.add_category_name = "".to_string();
                        app.add_category_empty = false;
                        game.save();
                        category.save();
                        app.loaded_category = Some(category);
                        app.num_splits_category = 0;
                    }
                }
                if ui.small_button("Cancel").clicked() {
                    app.add_category_name = "".to_string();
                    app.add_category_empty = false;
                    app.add_category_open = false;
                }
            })
        });
}

pub fn list(app: &mut HitSplit, ctx: &Context) {
    add_game(app, ctx);
    add_category(app, ctx);

    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        let selected_game: &mut String = &mut match app.loaded_game.clone() {
            None => "".to_owned(),
            Some(loaded) => loaded.uuid,
        };

        ui.heading("Splits");

        ui.horizontal(|ui| {
            ui.label("Game: ");
            egui::ComboBox::new("game", "")
                .selected_text(
                    (match app.loaded_game.clone() {
                        None => "".to_owned(),
                        Some(loaded) => loaded.name,
                    })
                    .to_string(),
                )
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    app.config.game_list.iter().for_each(|game| {
                        if ui
                            .selectable_value(
                                selected_game,
                                game.uuid.to_owned(),
                                game.name.to_owned(),
                            )
                            .clicked()
                        {
                            app.loaded_category = None;
                            app.loaded_game = Some(Game::load(selected_game.clone()));
                        }
                    });
                });

            if ui.small_button("Add game").clicked() {
                app.add_game_open = true;
            }
        });

        if let Some(g) = &mut app.loaded_game {
            let selected_category: &mut String = &mut match app.loaded_category.clone() {
                None => "".to_owned(),
                Some(cat) => cat.uuid,
            };

            ui.horizontal(|ui| {
                ui.label("Category: ");
                egui::ComboBox::new("category", "")
                    .selected_text(
                        (match app.loaded_category.clone() {
                            None => "".to_owned(),
                            Some(loaded) => loaded.name,
                        })
                        .to_string(),
                    )
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        g.categories.iter().for_each(|category| {
                            if ui
                                .selectable_value(
                                    selected_category,
                                    category.uuid.clone(),
                                    category.name.to_owned(),
                                )
                                .clicked()
                            {
                                app.loaded_category =
                                    Some(Category::load(selected_category.to_string()));
                                app.num_splits_category =
                                    app.loaded_category.as_ref().unwrap().splits.len() as u16;
                            };
                        });
                    });
                if ui.small_button("Add category").clicked() {
                    app.add_category_open = true;
                }
            });

            if let Some(c) = &mut app.loaded_category {
                ui.horizontal(|ui| {
                    ui.label("Number of splits: ");
                    integer_edit_field_u16(ui, &mut app.num_splits_category);
                });

                if ui.small_button("Create table").clicked() {
                    let cmp_splits: i16 = c.splits.len() as i16 - app.num_splits_category as i16;
                    if cmp_splits > 0 {
                        for _ in 0..cmp_splits {
                            c.splits.pop();
                        }
                    } else {
                        for _ in 0..cmp_splits.abs() {
                            let split = Split::default();
                            c.splits.push(split);
                        }
                    }
                    c.save();
                }

                ui.separator();
                ui.vertical(|ui| {
                    let table = egui_extras::TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::LEFT))
                        .resizable(true)
                        .column(egui_extras::Column::initial(100.0))
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .min_scrolled_height(0.0);

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
                            c.splits.iter_mut().for_each(|split: &mut Split| {
                                body.row(18., |mut row| {
                                    row.col(|ui| {
                                        ui.add(
                                            egui::TextEdit::singleline(&mut split.name)
                                                .desired_width(f32::MAX),
                                        );
                                    });
                                    row.col(|ui| {
                                        integer_edit_field_u16(ui, &mut split.hits);
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            (i32::from(split.hits) - i32::from(split.pb))
                                                .to_string(),
                                        );
                                    });
                                    row.col(|ui| {
                                        integer_edit_field_u16(ui, &mut split.pb);
                                    });
                                });
                            });
                            body.row(24., |mut row| {
                                row.col(|ui| {
                                    ui.label("Total: ");
                                });
                                row.col(|ui| {
                                    let hits = c.splits.iter().map(|split| split.hits);
                                    ui.label(hits.sum::<u16>().to_string());
                                });
                                row.col(|ui| {
                                    let diffs = c
                                        .splits
                                        .iter()
                                        .map(|split| i32::from(split.hits) - i32::from(split.pb));
                                    ui.label(diffs.sum::<i32>().to_string());
                                });
                                row.col(|ui| {
                                    let pbs = c.splits.iter().map(|split| split.pb);
                                    ui.label(pbs.sum::<u16>().to_string());
                                });
                            });
                        });
                });

                ui.horizontal(|ui| {
                    if ui.button("Save splits").clicked() {
                        c.save();
                    }
                    if ui.button("Open HitSplit counter").clicked() {
                        app.show_hit_counter = true;
                    }
                });
            } else {
                ui.separator();
                ui.heading("You must select a category to start");
            }
        } else {
            ui.separator();
            ui.heading("You must select a game to start");
        }
    });
}
