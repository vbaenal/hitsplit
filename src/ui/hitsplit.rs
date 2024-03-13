use super::{
    counter::counter,
    panels::{left_panel, list::list, settings::configuration, Pages},
};
use crate::{
    run::{category::Category, game::Game},
    settings::{
        config::Config,
        shortcut::{shortcut_handler, Shortcut, ShortcutAction},
    },
};
use eframe::{egui::Visuals, Storage};
use egui_file::FileDialog;
use global_hotkey::{hotkey::Code, GlobalHotKeyManager};
use std::{path::PathBuf, time::Duration};

pub struct HitSplit {
    pub config: Config,
    pub shortcut: Option<Shortcut>,
    pub num_splits_category: u16,
    pub open_page: Pages,
    pub add_game_name: String,
    pub add_game_open: bool,
    pub add_game_empty: bool,
    pub add_category_name: String,
    pub add_category_open: bool,
    pub add_category_empty: bool,
    pub loaded_game: Option<Game>,
    pub loaded_category: Option<Category>,
    pub selected_split: u16,
    pub show_hit_counter: bool,
    pub hotkey_manager: Option<GlobalHotKeyManager>,
    pub capturing: Option<ShortcutAction>,
    pub opened_file: Option<PathBuf>,
    pub open_file_dialog: Option<FileDialog>,
    pub change_split_img: Option<String>,
}

impl Clone for HitSplit {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            shortcut: self.shortcut.clone(),
            num_splits_category: self.num_splits_category,
            open_page: self.open_page.clone(),
            add_game_name: self.add_game_name.clone(),
            add_game_open: self.add_game_open,
            add_game_empty: self.add_game_empty,
            add_category_name: self.add_category_name.clone(),
            add_category_open: self.add_category_open,
            add_category_empty: self.add_category_empty,
            loaded_game: self.loaded_game.clone(),
            loaded_category: self.loaded_category.clone(),
            selected_split: self.selected_split,
            show_hit_counter: self.show_hit_counter,
            hotkey_manager: None,
            capturing: self.capturing,
            opened_file: self.opened_file.clone(),
            open_file_dialog: None,
            change_split_img: None,
        }
    }
}

impl Default for HitSplit {
    fn default() -> Self {
        Self {
            config: Default::default(),
            shortcut: Some(Default::default()),
            num_splits_category: 0,
            open_page: Pages::List,
            add_game_name: "".to_string(),
            add_game_open: false,
            add_game_empty: false,
            add_category_name: "".to_string(),
            add_category_open: false,
            add_category_empty: false,
            loaded_game: None,
            loaded_category: None,
            selected_split: 0,
            show_hit_counter: Default::default(),
            hotkey_manager: None,
            capturing: None,
            opened_file: None,
            open_file_dialog: None,
            change_split_img: None,
        }
    }
}

impl HitSplit {
    fn add_hotkey(&mut self, code: Code) {
        self.hotkey_manager
            .as_ref()
            .unwrap()
            .register(Shortcut::code_to_hotkey(code))
            .unwrap();
    }

    fn manage_hotkeys(&mut self) {
        self.hotkey_manager = Some(GlobalHotKeyManager::new().unwrap());
        if self.shortcut.is_some() && self.hotkey_manager.is_some() {
            self.shortcut
                .clone()
                .unwrap()
                .0
                .iter()
                .for_each(|&c| self.add_hotkey(c));
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: HitSplit = HitSplit {
            config: Config::load(),
            shortcut: Some(Shortcut::load()),
            ..Default::default()
        };
        app.manage_hotkeys();

        cc.egui_ctx.set_visuals(if app.config.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        });

        egui_extras::install_image_loaders(&cc.egui_ctx);
        app
    }
}

impl eframe::App for HitSplit {
    fn save(&mut self, _storage: &mut dyn Storage) {
        if self.config.autosave {
            self.config.save();
            self.shortcut.as_ref().unwrap().save();
            if let Some(cat) = &self.loaded_category {
                cat.save();
            }
        }
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        Duration::new(self.config.autosave_interval, 0)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.config.dark_mode = (*ctx.style()).clone().visuals.dark_mode;
        if let Some(sa) = &self.clone().capturing {
            if let Some(key) = ctx.input(|i| i.clone().keys_down.into_iter().last()) {
                ShortcutAction::change_shortcut(self, sa, key);
                self.manage_hotkeys();
                self.capturing = None;
            }
        }

        shortcut_handler(self);
        left_panel(self, ctx);

        match self.open_page {
            Pages::List => list(self, ctx),
            Pages::Settings => configuration(self, ctx),
        }

        if self.show_hit_counter {
            counter(self, ctx);
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if !self.config.autosave {
            self.config.save();
            self.shortcut.as_ref().unwrap().save();
            if let Some(cat) = &self.loaded_category {
                cat.save();
            }
        }
    }
}
