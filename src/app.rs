use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use eframe::{egui::Visuals, Storage};
use egui::Color32;
use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{image_button, integer_edit_field, integer_edit_field_u64};

#[derive(Serialize, Deserialize, Clone)]
struct Split {
    icon_path: String,
    name: String,
    hits: u16,
    pb: u16,
    real_time: Duration,
}

impl Split {
    fn new() -> Self {
        Self {
            icon_path: "".to_string(),
            name: "".to_string(),
            hits: 0,
            pb: 0,
            real_time: Duration::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Category {
    uuid: String,
    name: String,
    splits: Vec<Split>,
}

impl Category {
    fn by_name(uuid: String, name: String) -> Self {
        Category {
            uuid: uuid,
            name: name,
            splits: Vec::new(),
        }
    }

    fn load_category(uuid: String) -> Self {
        let game_json: String =
            match std::fs::read_to_string(format!("config/categories/{uuid}.json")) {
                Err(_) => "".to_string(),
                Ok(f) => f,
            };

        serde_json::from_str(game_json.as_str()).unwrap()
    }

    fn save_category(&self) {
        let category_str = serde_json::to_string(&self).unwrap();
        let _ = std::fs::write(
            format!("config/categories/{}.json", self.uuid),
            category_str,
        );
    }

    fn save_splits(&mut self, splits: Vec<Split>) {
        for split in splits {
            self.splits.push(split);
        }
        self.save_category();
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct SmallCategory {
    uuid: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Game {
    uuid: String,
    name: String,
    hitless_game: bool,
    hitless_id: u32,
    categories: Vec<SmallCategory>,
}

impl Game {
    fn by_name(uuid: String, name: String) -> Self {
        Game {
            uuid: uuid,
            name: name,
            hitless_game: false,
            hitless_id: 0,
            categories: Vec::new(),
        }
    }

    fn save_game(&self) {
        let game_str = serde_json::to_string(&self).unwrap();
        let _ = std::fs::write(format!("config/games/{}.json", self.uuid), game_str);
    }

    fn load_game(uuid: String) -> Self {
        let game_json: String = match std::fs::read_to_string(format!("config/games/{uuid}.json")) {
            Err(_) => "".to_string(),
            Ok(f) => f,
        };

        serde_json::from_str(game_json.as_str()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct SmallGame {
    uuid: String,
    name: String,
}

#[derive(Clone)]
struct Keybindings {
    prev_split: HotKey,
    next_split: HotKey,
    add_hit: HotKey,
    sub_hit: HotKey,
    reset: HotKey,
    set_pb: HotKey,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            prev_split: HotKey::new(None, Code::Numpad8),
            next_split: HotKey::new(None, Code::Numpad2),
            add_hit: HotKey::new(None, Code::Numpad7),
            sub_hit: HotKey::new(None, Code::Numpad9),
            reset: HotKey::new(None, Code::Numpad5),
            set_pb: HotKey::new(None, Code::Numpad3),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    dark_mode: bool,
    next_split_as_reset: bool,
    autosave: bool,
    autosave_interval: u64,
    game_list: Vec<SmallGame>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dark_mode: true,
            next_split_as_reset: true,
            autosave: true,
            autosave_interval: 5,
            game_list: Vec::new(),
        }
    }
}

impl Config {
    fn save_config(&mut self) {
        let config_str = serde_json::to_string(self).unwrap();
        let _ = std::fs::write("config/config.json", config_str);
    }

    fn load_config() -> Self {
        match std::fs::read_dir("config") {
            Err(_) => {
                let _ = std::fs::create_dir("config");
                true
            }
            Ok(_) => true,
        };

        match std::fs::read_dir("config/games") {
            Err(_) => {
                let _ = std::fs::create_dir("config/games");
                true
            }
            Ok(_) => true,
        };

        match std::fs::read_dir("config/categories") {
            Err(_) => {
                let _ = std::fs::create_dir("config/categories");
                true
            }
            Ok(_) => true,
        };

        let config_json: String = match std::fs::read_to_string("config/config.json") {
            Err(_) => {
                let tmp: Config = Default::default();
                let config_str = serde_json::to_string(&tmp).unwrap();
                let _ = std::fs::write("config/config.json", config_str.clone());
                config_str
            }
            Ok(f) => f,
        };

        serde_json::from_str(config_json.as_str()).unwrap()
    }
}

pub struct HitSplit {
    config: Config,
    keybinding: Option<Keybindings>,
    selected_game: String,
    selected_category: String,
    game_str: String,
    category_str: String,
    num_splits_category: u16,
    open_page: String,
    add_game_name: String,
    add_game_open: bool,
    add_game_empty: bool,
    add_category_name: String,
    add_category_open: bool,
    add_category_empty: bool,
    loaded_game: Option<SmallGame>,
    loaded_category: Option<SmallCategory>,
    loaded_splits: Vec<Split>,
    selected_split: u16,
    show_hit_counter: Arc<AtomicBool>,
    hotkey_manager: Option<GlobalHotKeyManager>,
}

impl Clone for HitSplit {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            keybinding: self.keybinding.clone(),
            selected_game: self.selected_game.clone(),
            selected_category: self.selected_category.clone(),
            game_str: self.game_str.clone(),
            category_str: self.category_str.clone(),
            num_splits_category: self.num_splits_category.clone(),
            open_page: self.open_page.clone(),
            add_game_name: self.add_game_name.clone(),
            add_game_open: self.add_game_open.clone(),
            add_game_empty: self.add_game_empty.clone(),
            add_category_name: self.add_category_name.clone(),
            add_category_open: self.add_category_open.clone(),
            add_category_empty: self.add_category_empty.clone(),
            loaded_game: self.loaded_game.clone(),
            loaded_category: self.loaded_category.clone(),
            loaded_splits: self.loaded_splits.clone(),
            selected_split: self.selected_split.clone(),
            show_hit_counter: self.show_hit_counter.clone(),
            hotkey_manager: None,
        }
    }
}

impl Default for HitSplit {
    fn default() -> Self {
        Self {
            config: Default::default(),
            keybinding: Some(Default::default()),
            selected_game: "".to_string(),
            selected_category: "".to_string(),
            game_str: "".to_string(),
            category_str: "".to_string(),
            num_splits_category: 0,
            open_page: "list".to_string(),
            add_game_name: "".to_string(),
            add_game_open: false,
            add_game_empty: false,
            add_category_name: "".to_string(),
            add_category_open: false,
            add_category_empty: false,
            loaded_game: None,
            loaded_category: None,
            loaded_splits: Vec::new(),
            selected_split: 0,
            show_hit_counter: Default::default(),
            hotkey_manager: Some(GlobalHotKeyManager::new().unwrap()),
        }
    }
}

impl HitSplit {
    fn manage_hotkeys(&mut self) {
        if self.keybinding.is_some() && self.hotkey_manager.is_some() {
            self.hotkey_manager
                .as_ref()
                .unwrap()
                .register(self.keybinding.as_ref().unwrap().prev_split)
                .unwrap();
            self.hotkey_manager
                .as_ref()
                .unwrap()
                .register(self.keybinding.as_ref().unwrap().next_split)
                .unwrap();
            self.hotkey_manager
                .as_ref()
                .unwrap()
                .register(self.keybinding.as_ref().unwrap().sub_hit)
                .unwrap();
            self.hotkey_manager
                .as_ref()
                .unwrap()
                .register(self.keybinding.as_ref().unwrap().add_hit)
                .unwrap();
            self.hotkey_manager
                .as_ref()
                .unwrap()
                .register(self.keybinding.as_ref().unwrap().reset)
                .unwrap();
            self.hotkey_manager
                .as_ref()
                .unwrap()
                .register(self.keybinding.as_ref().unwrap().set_pb)
                .unwrap();
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: HitSplit = Default::default();
        app.config = Config::load_config();
        app.manage_hotkeys();

        cc.egui_ctx.set_visuals(if app.config.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        });

        egui_extras::install_image_loaders(&cc.egui_ctx);
        app
    }

    fn list_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let selected_game: &mut String = &mut self.selected_game;

            ui.heading("Splits");

            ui.horizontal(|ui| {
                ui.label("Game: ");
                egui::ComboBox::new("game", "")
                    .selected_text(format!(
                        "{}",
                        if selected_game == "" {
                            "".to_owned()
                        } else {
                            self.config
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
                        self.config.game_list.iter().for_each(|game| {
                            if ui
                                .selectable_value(
                                    selected_game,
                                    game.uuid.to_owned(),
                                    game.name.to_owned(),
                                )
                                .clicked()
                            {
                                self.game_str = match std::fs::read_to_string(format!(
                                    "config/games/{selected_game}.json"
                                )) {
                                    Err(_) => "".to_owned(),
                                    Ok(f) => f.clone(),
                                };
                                self.selected_category = "".to_string();
                                self.loaded_category = None;
                            }
                        });
                    });

                if ui.small_button("Add game").clicked() {
                    self.add_game_open = true;
                }
            });

            if selected_game != "" {
                let game: Game = serde_json::from_str(self.game_str.as_str()).unwrap();
                let selected_category: &mut String = &mut self.selected_category;

                self.loaded_game = Option::Some(SmallGame {
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
                                    self.category_str = match std::fs::read_to_string(format!(
                                        "config/categories/{}.json",
                                        selected_category
                                    )) {
                                        Err(_) => "".to_owned(),
                                        Ok(f) => f,
                                    };
                                    let tmp_cat: Category =
                                        serde_json::from_str(self.category_str.as_str()).unwrap();
                                    self.loaded_splits = tmp_cat.splits;
                                    self.num_splits_category = self.loaded_splits.len() as u16;
                                };
                            });
                        });
                    if ui.small_button("Add category").clicked() {
                        self.add_category_open = true;
                    }
                });

                if self.selected_category.clone() != "".to_owned() {
                    let mut category: Category =
                        serde_json::from_str(self.category_str.as_str()).unwrap();
                    self.loaded_category =
                        serde_json::from_str(self.category_str.as_str()).unwrap();

                    ui.horizontal(|ui| {
                        ui.label("Number of splits: ");
                        integer_edit_field(ui, &mut self.num_splits_category);
                    });

                    if ui.small_button("Create table").clicked() {
                        let cmp_splits: i16 =
                            self.loaded_splits.len() as i16 - self.num_splits_category as i16;
                        if cmp_splits > 0 {
                            for _ in 0..cmp_splits {
                                self.loaded_splits.pop();
                            }
                        } else {
                            for _ in 0..cmp_splits.abs() {
                                let split = Split::new();
                                self.loaded_splits.push(split);
                            }
                        }
                        category.save_category();
                        self.category_str = match std::fs::read_to_string(format!(
                            "config/categories/{}.json",
                            self.selected_category.clone()
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
                                self.loaded_splits.iter_mut().for_each(|split: &mut Split| {
                                    body.row(18., |mut row| {
                                        row.col(|ui| {
                                            ui.add(
                                                egui::TextEdit::singleline(&mut split.name)
                                                    .desired_width(f32::MAX),
                                            );
                                        });
                                        row.col(|ui| {
                                            integer_edit_field(ui, &mut split.hits);
                                        });
                                        row.col(|ui| {
                                            ui.label(
                                                (i32::from(split.hits) - i32::from(split.pb))
                                                    .to_string(),
                                            );
                                        });
                                        row.col(|ui| {
                                            integer_edit_field(ui, &mut split.pb);
                                        });
                                    });
                                });
                                body.row(24., |mut row| {
                                    row.col(|ui| {
                                        ui.label("Total: ");
                                    });
                                    row.col(|ui| {
                                        let hits =
                                            self.loaded_splits.iter().map(|split| split.hits);
                                        ui.label(hits.sum::<u16>().to_string());
                                    });
                                    row.col(|ui| {
                                        let diffs = self.loaded_splits.iter().map(|split| {
                                            i32::from(split.hits) - i32::from(split.pb)
                                        });
                                        ui.label(diffs.sum::<i32>().to_string());
                                    });
                                    row.col(|ui| {
                                        let pbs = self.loaded_splits.iter().map(|split| split.pb);
                                        ui.label(pbs.sum::<u16>().to_string());
                                    });
                                });
                            });
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Save splits").clicked() {
                            category.save_splits(self.loaded_splits.to_vec());
                        }
                        if ui.button("Open HitSplit counter").clicked() {
                            self.show_hit_counter.store(true, Ordering::Relaxed);
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

    fn config_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Configuration");
            ui.horizontal(|ui| {
                ui.label("Visual mode: ");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
            ui.horizontal(|ui| {
                ui.label("Autosave: ");
                ui.checkbox(&mut self.config.autosave, "");
            });
            if self.config.autosave {
                ui.horizontal(|ui| {
                    ui.label("Autosave interval: ");
                    integer_edit_field_u64(ui, &mut self.config.autosave_interval, 30.0);
                    ui.label("seconds");
                });
            }
            ui.horizontal(|ui| {
                ui.label("Next split as reset on last split: ");
                ui.checkbox(&mut self.config.next_split_as_reset, "");
            });

            ui.separator();
            ui.heading("Keybindings");
            ui.label("Work in progress. These are the default keys:");
            ui.horizontal(|ui| {
                ui.label("Previous split: ");
                ui.label("NUMPAD8");
            });
            ui.horizontal(|ui| {
                ui.label("Next split: ");
                ui.label("NUMPAD2");
            });
            ui.horizontal(|ui| {
                ui.label("Add hit: ");
                ui.label("NUMPAD7");
            });
            ui.horizontal(|ui| {
                ui.label("Substract hit: ");
                ui.label("NUMPAD9");
            });
            ui.horizontal(|ui| {
                ui.label("Reset: ");
                ui.label("NUMPAD5");
            });
            ui.horizontal(|ui| {
                ui.label("Set current table as PB: ");
                ui.label("NUMPAD3");
            });

            if ui.button("Save config").clicked() {
                self.config.save_config();
                if self.selected_category != "" {
                    let mut tmp_cat = Category::load_category(self.selected_category.clone());
                    tmp_cat.splits = self.loaded_splits.clone();
                    tmp_cat.save_category();
                }
            }
        });
    }
}

impl eframe::App for HitSplit {
    fn save(&mut self, _storage: &mut dyn Storage) {
        if self.config.autosave {
            self.config.save_config();
            if self.selected_category != "" {
                let mut tmp_cat = Category::load_category(self.selected_category.clone());
                tmp_cat.splits = self.loaded_splits.clone();
                tmp_cat.save_category();
            }
        }
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        Duration::new(self.config.autosave_interval, 0)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.config.dark_mode = (*ctx.style()).clone().visuals.dark_mode;

        let app = self.clone();
        if self.show_hit_counter.load(Ordering::Relaxed) {
            let receiver = GlobalHotKeyEvent::receiver();
            if let Ok(event) = receiver.try_recv() {
                if event.id == self.keybinding.clone().unwrap().prev_split.id()
                    && event.state == HotKeyState::Pressed
                {
                    if self.selected_split > 0 {
                        self.selected_split -= 1;
                    }
                }
                if event.id == self.keybinding.clone().unwrap().next_split.id()
                    && event.state == HotKeyState::Pressed
                {
                    if self.loaded_splits.len() > self.selected_split as usize + 1 {
                        self.selected_split += 1;
                    } else if self.config.next_split_as_reset
                        && self.loaded_splits.len() == self.selected_split as usize + 1
                    {
                        self.selected_split = 0;
                        let pbs = self.loaded_splits.iter().map(|split| split.pb).sum::<u16>();
                        let hits = self
                            .loaded_splits
                            .iter()
                            .map(|split| split.hits)
                            .sum::<u16>();
                        self.loaded_splits.iter_mut().for_each(|split| {
                            if pbs > hits {
                                split.pb = split.hits;
                            }
                            split.hits = 0;
                        });
                    }
                } else if event.id == self.keybinding.clone().unwrap().sub_hit.id()
                    && event.state == HotKeyState::Pressed
                {
                    let split = self
                        .loaded_splits
                        .get_mut(self.selected_split as usize)
                        .unwrap();
                    if split.hits > 0 {
                        split.hits -= 1;
                    }
                } else if event.id == self.keybinding.clone().unwrap().add_hit.id()
                    && event.state == HotKeyState::Pressed
                {
                    self.loaded_splits
                        .get_mut(self.selected_split as usize)
                        .unwrap()
                        .hits += 1;
                } else if event.id == self.keybinding.clone().unwrap().reset.id()
                    && event.state == HotKeyState::Pressed
                {
                    self.loaded_splits.iter_mut().for_each(|split| {
                        split.hits = 0;
                    });
                    self.selected_split = 0;
                } else if event.id == self.keybinding.clone().unwrap().set_pb.id()
                    && event.state == HotKeyState::Pressed
                {
                    self.loaded_splits.iter_mut().for_each(|split| {
                        split.pb = split.hits;
                    });
                }
            }
            let show_hit_counter = self.show_hit_counter.clone();
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("hitsplit_counter"),
                egui::ViewportBuilder::default()
                    .with_title("HitSplit Counter")
                    .with_resizable(true)
                    .with_inner_size([280.0, 600.0]),
                move |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(app.loaded_game.unwrap().name);
                        });
                        if app.loaded_category.is_some() {
                            ui.vertical_centered(|ui| {
                                ui.label(app.loaded_category.unwrap().name);
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
                                    app.loaded_splits.iter().enumerate().for_each(|(i, split)| {
                                        let mut label_color = color.clone();
                                        if i <= app.selected_split as usize {
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
                                            if i == app.selected_split as usize {
                                                name = format!("> {}", name);
                                            }
                                            row.col(|ui| {
                                                ui.colored_label(label_color, name);
                                            });
                                            row.col(|ui| {
                                                ui.colored_label(
                                                    label_color,
                                                    split.hits.to_string(),
                                                );
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
                                            let hits =
                                                app.loaded_splits.iter().map(|split| split.hits);
                                            ui.colored_label(color, hits.sum::<u16>().to_string());
                                        });
                                        row.col(|ui| {
                                            let diffs = app.loaded_splits.iter().map(|split| {
                                                i32::from(split.hits) - i32::from(split.pb)
                                            });
                                            ui.colored_label(color, diffs.sum::<i32>().to_string());
                                        });
                                        row.col(|ui| {
                                            let pbs =
                                                app.loaded_splits.iter().map(|split| split.pb);
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

        egui::Window::new("Add game")
            .fixed_pos(&[50., 50.])
            .resizable(false)
            .open(&mut self.add_game_open.clone())
            .title_bar(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| ui.label("Enter the name of the game"));
                if self.add_game_empty {
                    ui.colored_label(Color32::from_rgb(250, 8, 8), "You must enter a game name!");
                }
                ui.horizontal(|ui| ui.add(egui::TextEdit::singleline(&mut self.add_game_name)));
                ui.horizontal(|ui| {
                    if ui.small_button("Add").clicked() {
                        if self.add_game_name.eq("") {
                            self.add_game_empty = true;
                        } else {
                            let uuid: String = Uuid::new_v4().to_string();
                            self.config.game_list.push(SmallGame {
                                uuid: uuid.clone(),
                                name: self.add_game_name.clone(),
                            });
                            let game = Game::by_name(uuid, self.add_game_name.clone());
                            self.selected_game = game.uuid.clone();
                            game.save_game();
                            self.add_game_name = "".to_string();
                            self.add_game_empty = false;
                            self.add_game_open = false;
                            self.game_str = match std::fs::read_to_string(format!(
                                "config/games/{}.json",
                                self.selected_game
                            )) {
                                Err(_) => "".to_owned(),
                                Ok(f) => f.clone(),
                            };
                            self.selected_category = "".to_string();
                            self.config.save_config();
                            self.loaded_splits = Vec::new();
                            self.num_splits_category = 0;
                            self.loaded_category = None;
                        }
                    }
                    if ui.small_button("Cancel").clicked() {
                        self.add_game_name = "".to_string();
                        self.add_game_empty = false;
                        self.add_game_open = false;
                    }
                })
            });

        egui::Window::new("Add category")
            .fixed_pos(&[50., 50.])
            .resizable(false)
            .open(&mut self.add_category_open.clone())
            .title_bar(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| ui.label("Enter the name of the category"));
                if self.add_category_empty {
                    ui.colored_label(
                        Color32::from_rgb(250, 8, 8),
                        "You must enter a category name!",
                    );
                }
                ui.horizontal(|ui| ui.add(egui::TextEdit::singleline(&mut self.add_category_name)));
                ui.horizontal(|ui| {
                    if ui.small_button("Add").clicked() {
                        if self.add_category_name.eq("") {
                            self.add_category_empty = true;
                        } else {
                            let mut game: Game = Game::load_game(self.selected_game.clone());
                            let uuid: String = Uuid::new_v4().to_string();
                            self.add_category_open = false;
                            game.categories.push(SmallCategory {
                                uuid: uuid.clone(),
                                name: self.add_category_name.clone(),
                            });
                            let category =
                                Category::by_name(uuid.clone(), self.add_category_name.clone());
                            self.add_category_name = "".to_string();
                            self.add_category_empty = false;
                            self.selected_category = uuid;
                            game.save_game();
                            self.game_str = match std::fs::read_to_string(format!(
                                "config/games/{}.json",
                                self.selected_game
                            )) {
                                Err(_) => "".to_owned(),
                                Ok(f) => f.clone(),
                            };
                            category.save_category();
                            self.category_str = match std::fs::read_to_string(format!(
                                "config/categories/{}.json",
                                self.selected_category.clone()
                            )) {
                                Err(_) => "".to_owned(),
                                Ok(f) => f,
                            };
                            self.loaded_splits = Vec::new();
                            self.num_splits_category = 0;
                        }
                    }
                    if ui.small_button("Cancel").clicked() {
                        self.add_category_name = "".to_string();
                        self.add_category_empty = false;
                        self.add_category_open = false;
                    }
                })
            });

        egui::SidePanel::left("left_panel")
            .exact_width(72.)
            .resizable(false)
            .show(ctx, |ui| {
                let list_button;
                let config_button;
                //Get Style
                let style: egui::Style = (*ctx.style()).clone();
                if style.visuals.dark_mode {
                    list_button = egui::include_image!("../assets/dark_mode/list.svg");
                    config_button = egui::include_image!("../assets/dark_mode/config.svg");
                } else {
                    list_button = egui::include_image!("../assets/light_mode/list.svg");
                    config_button = egui::include_image!("../assets/light_mode/config.svg");
                }

                if ui.add(image_button(list_button)).clicked() {
                    self.open_page = "list".to_owned();
                } else if ui.add(image_button(config_button)).clicked() {
                    self.open_page = "config".to_owned();
                }
            });

        if self.open_page == "list".to_owned() {
            self.list_panel(ctx);
        } else if self.open_page == "config".to_owned() {
            self.config_panel(ctx);
        }

        if self.show_hit_counter.load(Ordering::Relaxed) {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}
}
