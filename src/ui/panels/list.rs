use std::sync::atomic::Ordering;

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
        .fixed_pos(&[50., 50.])
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
                        app.selected_game = game.uuid.clone();
                        game.save();
                        app.add_game_name = "".to_string();
                        app.add_game_empty = false;
                        app.add_game_open = false;
                        app.game_str = match std::fs::read_to_string(format!(
                            "config/games/{}.json",
                            app.selected_game
                        )) {
                            Err(_) => "".to_owned(),
                            Ok(f) => f.clone(),
                        };
                        app.selected_category = "".to_string();
                        app.config.save();
                        app.loaded_splits = Vec::new();
                        app.num_splits_category = 0;
                        app.loaded_category = None;
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
        .fixed_pos(&[50., 50.])
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
                        let mut game: Game = Game::load(app.selected_game.clone());
                        let uuid: String = Uuid::new_v4().to_string();
                        app.add_category_open = false;
                        game.categories.push(SmallCategory {
                            uuid: uuid.clone(),
                            name: app.add_category_name.clone(),
                        });
                        let category = Category::new(uuid.clone(), app.add_category_name.clone());
                        app.add_category_name = "".to_string();
                        app.add_category_empty = false;
                        app.selected_category = uuid;
                        game.save();
                        app.game_str = match std::fs::read_to_string(format!(
                            "config/games/{}.json",
                            app.selected_game
                        )) {
                            Err(_) => "".to_owned(),
                            Ok(f) => f.clone(),
                        };
                        category.save();
                        app.category_str = match std::fs::read_to_string(format!(
                            "config/categories/{}.json",
                            app.selected_category.clone()
                        )) {
                            Err(_) => "".to_owned(),
                            Ok(f) => f,
                        };
                        app.loaded_splits = Vec::new();
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
        let selected_game: &mut String = &mut app.selected_game;

        ui.heading("Splits");

        ui.horizontal(|ui| {
            ui.label("Game: ");
            egui::ComboBox::new("game", "")
                .selected_text(format!(
                    "{}",
                    if selected_game == "" {
                        "".to_owned()
                    } else {
                        app.config
                            .game_list
                            .iter()
                            .find(|g| g.uuid.eq(selected_game))
                            .unwrap()
                            .name
                            .to_owned()
                    }
                ))
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
                            app.game_str = match std::fs::read_to_string(format!(
                                "config/games/{selected_game}.json"
                            )) {
                                Err(_) => "".to_owned(),
                                Ok(f) => f.clone(),
                            };
                            app.selected_category = "".to_string();
                            app.loaded_category = None;
                        }
                    });
                });

            if ui.small_button("Add game").clicked() {
                app.add_game_open = true;
            }
        });

        if selected_game != "" {
            let game: Game = serde_json::from_str(app.game_str.as_str()).unwrap();
            let selected_category: &mut String = &mut app.selected_category;

            app.loaded_game = Option::Some(SmallGame {
                uuid: game.uuid,
                name: game.name,
            });

            ui.horizontal(|ui| {
                ui.label("Category: ");
                egui::ComboBox::new("category", "")
                    .selected_text(format!(
                        "{}",
                        if selected_category.clone() == "" {
                            "".to_owned()
                        } else {
                            match game
                                .categories
                                .iter()
                                .find(|cat| cat.uuid == selected_category.clone())
                            {
                                None => "".to_owned(),
                                Some(cat) => cat.name.clone(),
                            }
                        }
                    ))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        game.categories.iter().for_each(|category| {
                            if ui
                                .selectable_value(
                                    selected_category,
                                    category.uuid.clone(),
                                    category.name.to_owned(),
                                )
                                .clicked()
                            {
                                app.category_str = match std::fs::read_to_string(format!(
                                    "config/categories/{}.json",
                                    selected_category
                                )) {
                                    Err(_) => "".to_owned(),
                                    Ok(f) => f,
                                };
                                let tmp_cat: Category =
                                    serde_json::from_str(app.category_str.as_str()).unwrap();
                                app.loaded_splits = tmp_cat.splits;
                                app.num_splits_category = app.loaded_splits.len() as u16;
                            };
                        });
                    });
                if ui.small_button("Add category").clicked() {
                    app.add_category_open = true;
                }
            });

            if app.selected_category.clone() != "".to_owned() {
                let mut category: Category =
                    serde_json::from_str(app.category_str.as_str()).unwrap();
                app.loaded_category = serde_json::from_str(app.category_str.as_str()).unwrap();

                ui.horizontal(|ui| {
                    ui.label("Number of splits: ");
                    integer_edit_field_u16(ui, &mut app.num_splits_category);
                });

                if ui.small_button("Create table").clicked() {
                    let cmp_splits: i16 =
                        app.loaded_splits.len() as i16 - app.num_splits_category as i16;
                    if cmp_splits > 0 {
                        for _ in 0..cmp_splits {
                            app.loaded_splits.pop();
                        }
                    } else {
                        for _ in 0..cmp_splits.abs() {
                            let split = Split::new();
                            app.loaded_splits.push(split);
                        }
                    }
                    category.save();
                    app.category_str = match std::fs::read_to_string(format!(
                        "config/categories/{}.json",
                        app.selected_category.clone()
                    )) {
                        Err(_) => "".to_owned(),
                        Ok(f) => f,
                    };
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
                            app.loaded_splits.iter_mut().for_each(|split: &mut Split| {
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
                                    let hits = app.loaded_splits.iter().map(|split| split.hits);
                                    ui.label(hits.sum::<u16>().to_string());
                                });
                                row.col(|ui| {
                                    let diffs = app
                                        .loaded_splits
                                        .iter()
                                        .map(|split| i32::from(split.hits) - i32::from(split.pb));
                                    ui.label(diffs.sum::<i32>().to_string());
                                });
                                row.col(|ui| {
                                    let pbs = app.loaded_splits.iter().map(|split| split.pb);
                                    ui.label(pbs.sum::<u16>().to_string());
                                });
                            });
                        });
                });

                ui.horizontal(|ui| {
                    if ui.button("Save splits").clicked() {
                        category.save_splits(app.loaded_splits.to_vec());
                    }
                    if ui.button("Open HitSplit counter").clicked() {
                        app.show_hit_counter.store(true, Ordering::Relaxed);
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
