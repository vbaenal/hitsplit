use std::{ffi::OsStr, path::Path};

use egui::{Color32, Context};
use uuid::Uuid;

use crate::{
    get_file_dialog,
    run::{
        category::{Category, SmallCategory},
        game::{Game, SmallGame},
        split::Split,
    },
    settings::columns::{Column, ColumnVec},
    ui::{
        functions::{image_button, numeric_edit_field_u16, numeric_edit_field_usize},
        ChangeImage,
    },
    HitSplit,
};

const FILE_EXTENSIONS: [Option<&'static str>; 3] = [Some("png"), Some("jpg"), Some("jpeg")];

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
                        if let Err(e) = game.save() {
                            app.error = e;
                        }
                        app.add_game_name = "".to_string();
                        app.add_game_empty = false;
                        app.add_game_open = false;
                        if app.config.save().is_err() {
                            ui.colored_label(
                                Color32::from_rgb(250, 16, 16),
                                "Could not save config",
                            );
                        };
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

fn modify_game(app: &mut HitSplit, ctx: &Context) {
    egui::Window::new("Modify game name")
        .fixed_pos([50., 50.])
        .resizable(false)
        .open(&mut app.modify_game_open.clone())
        .title_bar(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| ui.label("Enter the new name of the game"));
            if app.add_game_empty {
                ui.colored_label(Color32::from_rgb(250, 8, 8), "You must enter a game name!");
            }
            ui.horizontal(|ui| ui.add(egui::TextEdit::singleline(&mut app.add_game_name)));
            ui.horizontal(|ui| {
                if ui.small_button("Change name").clicked() {
                    if app.add_game_name.eq("") {
                        app.add_game_empty = true;
                    } else if let Some(game) = app.loaded_game.as_mut() {
                        game.change_name(&app.add_game_name);
                        if let Some(game) = app
                            .config
                            .game_list
                            .iter_mut()
                            .find(|sg| sg.uuid == game.uuid)
                        {
                            game.change_name(&app.add_game_name)
                        };
                        if let Err(e) = game.save() {
                            app.error = e;
                        }
                        app.add_game_name = "".to_string();
                        app.add_game_empty = false;
                        app.modify_game_open = false;
                        if app.config.save().is_err() {
                            ui.colored_label(
                                Color32::from_rgb(250, 16, 16),
                                "Could not save config",
                            );
                        };
                    }
                }
                if ui.small_button("Cancel").clicked() {
                    app.add_game_name = "".to_string();
                    app.add_game_empty = false;
                    app.modify_game_open = false;
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
                    } else if let Some(game) = app.loaded_game.as_mut() {
                        let uuid: String = Uuid::new_v4().to_string();
                        app.add_category_open = false;
                        game.categories.push(SmallCategory {
                            uuid: uuid.clone(),
                            name: app.add_category_name.clone(),
                        });
                        let category = Category::new(uuid.clone(), app.add_category_name.clone());
                        app.add_category_name = "".to_string();
                        app.add_category_empty = false;
                        if let Err(e) = game.save() {
                            app.error = e;
                        }
                        if let Err(e) = category.save() {
                            app.error = e;
                        }
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

fn modify_category(app: &mut HitSplit, ctx: &Context) {
    egui::Window::new("Modify category name")
        .fixed_pos([50., 50.])
        .resizable(false)
        .open(&mut app.modify_category_open.clone())
        .title_bar(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| ui.label("Enter the new name of the category"));
            if app.add_category_empty {
                ui.colored_label(
                    Color32::from_rgb(250, 8, 8),
                    "You must enter a category name!",
                );
            }
            ui.horizontal(|ui| ui.add(egui::TextEdit::singleline(&mut app.add_category_name)));
            ui.horizontal(|ui| {
                if ui.small_button("Change name").clicked() {
                    if app.add_category_name.eq("") {
                        app.add_category_empty = true;
                    } else if let Some(game) = app.loaded_game.as_mut() {
                        if let Some(category) = app.loaded_category.as_mut() {
                            category.change_name(&app.add_category_name);
                            if let Some(cat) = game
                                .categories
                                .iter_mut()
                                .find(|sg| sg.uuid == category.uuid)
                            {
                                cat.change_name(&app.add_category_name);
                            };
                            if let Err(e) = category.save() {
                                app.error = e;
                            }
                            app.add_category_name = "".to_string();
                            app.add_category_empty = false;
                            if let Err(e) = game.save() {
                                app.error = e;
                            }
                            if let Err(e) = category.save() {
                                app.error = e;
                            }
                            app.modify_category_open = false;
                        };
                    }
                }
                if ui.small_button("Cancel").clicked() {
                    app.add_category_name = "".to_string();
                    app.add_category_empty = false;
                    app.modify_category_open = false;
                }
            })
        });
}

fn column_check(ui: &mut egui::Ui, columns: &mut ColumnVec, column: &Column) {
    let has_column = columns.contains(column);
    let mut has_column_mut = has_column;
    ui.checkbox(&mut has_column_mut, "");
    if has_column != has_column_mut {
        if has_column_mut {
            columns.push(column);
        } else {
            columns.remove(column);
        }
    }
}

pub fn list(app: &mut HitSplit, ctx: &Context) {
    add_game(app, ctx);
    modify_game(app, ctx);
    add_category(app, ctx);
    modify_category(app, ctx);

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
                            app.loaded_game = match Game::load(selected_game.clone()) {
                                Ok(g) => Some(g),
                                Err(e) => {
                                    app.error = e;
                                    None
                                }
                            };
                        }
                    });
                });

            if ui.small_button("Add game").clicked() {
                app.add_game_open = true;
            }
            if let Some(g) = &mut app.loaded_game {
                if ui.small_button("Modify game name").clicked() {
                    app.modify_game_open = true;
                }
                if ui.small_button("Set game image").clicked() {
                    let filter = Box::new({
                        move |path: &Path| -> bool {
                            FILE_EXTENSIONS
                                .iter()
                                .any(|fe| fe.map(OsStr::new) == path.extension())
                        }
                    });
                    let mut dialog = get_file_dialog(g.icon_path.clone()).show_files_filter(filter);
                    dialog.open();
                    app.change_image = Some(ChangeImage::Game);
                    app.open_file_dialog = Some(dialog);
                }
                if ui.small_button("Clear game image").clicked() {
                    g.icon_path = None;
                }
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
                                    match Category::load(selected_category.to_string()) {
                                        Ok(mut c) => {
                                            app.num_splits_category = c.splits.len();
                                            c.splits.iter_mut().for_each(|s| {
                                                if s.uuid.is_none() {
                                                    s.uuid = Some(Uuid::new_v4().to_string());
                                                }
                                            });
                                            Some(c)
                                        }
                                        Err(e) => {
                                            app.error = e;
                                            None
                                        }
                                    };
                            };
                        });
                    });
                if ui.small_button("Add category").clicked() {
                    app.add_category_open = true;
                }
                if let Some(c) = &mut app.loaded_category {
                    if ui.small_button("Modify category name").clicked() {
                        app.modify_category_open = true;
                    }
                    if ui.small_button("Set category image").clicked() {
                        let filter = Box::new({
                            move |path: &Path| -> bool {
                                FILE_EXTENSIONS
                                    .iter()
                                    .any(|fe| fe.map(OsStr::new) == path.extension())
                            }
                        });
                        let mut dialog =
                            get_file_dialog(c.icon_path.clone()).show_files_filter(filter);
                        dialog.open();
                        app.change_image = Some(ChangeImage::Category);
                        app.open_file_dialog = Some(dialog);
                    }
                    if ui.small_button("Clear category image").clicked() {
                        c.icon_path = None;
                    }
                }
            });

            if let Some(c) = &mut app.loaded_category {
                ui.horizontal(|ui| {
                    ui.label("Number of splits: ");
                    numeric_edit_field_usize(ui, &mut app.num_splits_category);
                });

                if ui.small_button("Create table").clicked() {
                    let cmp_splits: i16 = c.splits.len() as i16 - app.num_splits_category as i16;
                    if cmp_splits > 0 {
                        for _ in 0..cmp_splits {
                            c.splits.pop();
                        }
                    } else {
                        for _ in 0..cmp_splits.abs() {
                            let split = Split::new(Some(Uuid::new_v4().to_string()));
                            c.splits.push(split);
                        }
                    }
                    if let Err(e) = c.save() {
                        app.error = e;
                    }
                }

                ui.separator();
                ui.vertical(|ui| {
                    let table = egui_extras::TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::LEFT))
                        .resizable(true)
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::initial(100.0))
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::auto())
                        .column(egui_extras::Column::initial(24.0))
                        .column(egui_extras::Column::initial(24.0))
                        .min_scrolled_height(0.0);

                    table
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Image");
                                column_check(ui, &mut app.config.columns, &Column::Icon);
                            });
                            header.col(|ui| {
                                ui.strong("Name");
                                column_check(ui, &mut app.config.columns, &Column::SplitName);
                            });
                            header.col(|ui| {
                                ui.strong("Hits");
                                column_check(ui, &mut app.config.columns, &Column::Hits);
                            });
                            header.col(|ui| {
                                ui.strong("Diff");
                                column_check(ui, &mut app.config.columns, &Column::Difference);
                            });
                            header.col(|ui| {
                                ui.strong("PB");
                                column_check(ui, &mut app.config.columns, &Column::PersonalBest);
                            });
                            header.col(|ui| {
                                ui.strong("Chrono");
                                column_check(ui, &mut app.config.columns, &Column::Chrono);
                            });
                            header.col(|ui| {
                                ui.strong("Chrono Ac.");
                                column_check(ui, &mut app.config.columns, &Column::ChronoAcum);
                            });
                            header.col(|ui| {
                                ui.strong("");
                            });
                            header.col(|ui| {
                                ui.strong("");
                            });
                        })
                        .body(|mut body| {
                            c.splits.iter_mut().enumerate().for_each(|(i, split)| {
                                body.row(24., |mut row| {
                                    row.col(|ui| {
                                        if let Some(p) = &split.icon_path {
                                            if let Some(path) = p.to_str() {
                                                if ui
                                                    .add(image_button(
                                                        format!("file://{path}"),
                                                        16.0,
                                                        16.0,
                                                        0.0,
                                                    ))
                                                    .clicked()
                                                {
                                                    let filter = Box::new({
                                                        move |path: &Path| -> bool {
                                                            FILE_EXTENSIONS.iter().any(|fe| {
                                                                fe.map(OsStr::new)
                                                                    == path.extension()
                                                            })
                                                        }
                                                    });
                                                    let mut dialog =
                                                        get_file_dialog(Some(p.to_path_buf()))
                                                            .show_files_filter(filter);
                                                    dialog.open();
                                                    app.open_file_dialog = Some(dialog);
                                                    if let Some(uuid) = split.uuid.clone() {
                                                        app.change_image.clone_from(&Some(
                                                            ChangeImage::Split(uuid),
                                                        ));
                                                    }
                                                }
                                            };
                                        } else if ui.button("Add image").clicked() {
                                            let filter = Box::new({
                                                move |path: &Path| -> bool {
                                                    FILE_EXTENSIONS.iter().any(|fe| {
                                                        fe.map(OsStr::new) == path.extension()
                                                    })
                                                }
                                            });
                                            let mut dialog =
                                                get_file_dialog(None).show_files_filter(filter);
                                            dialog.open();
                                            app.open_file_dialog = Some(dialog);
                                            if let Some(uuid) = split.uuid.clone() {
                                                app.change_image
                                                    .clone_from(&Some(ChangeImage::Split(uuid)));
                                            }
                                        }
                                        if split.icon_path.is_some()
                                            && ui.button("Clear image").clicked()
                                        {
                                            split.clear_icon_path();
                                        }
                                    });
                                    row.col(|ui| {
                                        ui.add(
                                            egui::TextEdit::singleline(&mut split.name)
                                                .desired_width(f32::MAX),
                                        );
                                    });
                                    row.col(|ui| {
                                        numeric_edit_field_u16(ui, &mut split.hits);
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            (i32::from(split.hits) - i32::from(split.pb))
                                                .to_string(),
                                        );
                                    });
                                    row.col(|ui| {
                                        numeric_edit_field_u16(ui, &mut split.pb);
                                    });
                                    row.col(|_ui| {});
                                    row.col(|_ui| {});
                                    row.col(|ui| {
                                        if ui.button("➕").clicked() {
                                            app.add_split_under = Some(i);
                                        }
                                    });
                                    row.col(|ui| {
                                        if ui.button("➖").clicked() {
                                            app.delete_split = Some(i);
                                        }
                                    });
                                });
                            });
                            body.row(24., |mut row| {
                                row.col(|_| {});
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
                                row.col(|_ui| {});
                                row.col(|_ui| {});
                                row.col(|_ui| {});
                                row.col(|_ui| {});
                            });
                        });

                    if let Some(split) = app.delete_split {
                        c.splits.remove(split);
                        app.delete_split = None;
                        if let Err(e) = c.save() {
                            app.error = e;
                        }
                        app.num_splits_category = c.splits.len();
                    }

                    if let Some(split) = app.add_split_under {
                        c.splits.insert(split + 1, Split::default());
                        app.add_split_under = None;
                        if let Err(e) = c.save() {
                            app.error = e;
                        }
                        app.num_splits_category = c.splits.len();
                    }

                    if let Some(dialog) = &mut app.open_file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                if let Some(change) = &app.change_image {
                                    match change {
                                        ChangeImage::Game => g.icon_path = Some(file.to_path_buf()),
                                        ChangeImage::Category => {
                                            c.icon_path = Some(file.to_path_buf())
                                        }
                                        ChangeImage::Split(uuid) => {
                                            if let Some(split) = c.splits.iter_mut().find(|s| {
                                                if let Some(s_uuid) = s.uuid.clone() {
                                                    s_uuid == uuid.clone()
                                                } else {
                                                    false
                                                }
                                            }) {
                                                split.icon_path = Some(file.to_path_buf());
                                            }
                                        }
                                    };
                                }
                                app.change_image = None;
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Save splits").clicked() {
                        if let Err(e) = g.save() {
                            app.error = e;
                        }
                        if let Err(e) = c.save() {
                            app.error = e;
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Clear splits images").clicked() {
                        c.clear_icon_path();
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
