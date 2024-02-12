use super::{
    counter::counter,
    panels::{left_panel, list::list, settings::configuration, Pages},
};
use crate::{
    config::{
        config::Config,
        keybindings::{keybinding_handler, Keybindings},
    },
    run::{category::Category, game::Game},
};
use eframe::{egui::Visuals, Storage};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

pub struct HitSplit {
    pub config: Config,
    pub keybinding: Option<Keybindings>,
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
    pub show_hit_counter: Arc<AtomicBool>,
    pub hotkey_manager: Option<GlobalHotKeyManager>,
}

impl Clone for HitSplit {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            keybinding: self.keybinding.clone(),
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
            hotkey_manager: Some(GlobalHotKeyManager::new().unwrap()),
        }
    }
}

impl HitSplit {
    fn add_hotkey(&mut self, hotkey: HotKey) {
        self.hotkey_manager
            .as_ref()
            .unwrap()
            .register(hotkey)
            .unwrap();
    }

    fn manage_hotkeys(&mut self) {
        if self.keybinding.is_some() && self.hotkey_manager.is_some() {
            self.add_hotkey(self.keybinding.as_ref().unwrap().prev_split);
            self.add_hotkey(self.keybinding.as_ref().unwrap().next_split);
            self.add_hotkey(self.keybinding.as_ref().unwrap().sub_hit);
            self.add_hotkey(self.keybinding.as_ref().unwrap().add_hit);
            self.add_hotkey(self.keybinding.as_ref().unwrap().reset);
            self.add_hotkey(self.keybinding.as_ref().unwrap().set_pb);
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: HitSplit = Default::default();
        app.config = Config::load();
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
        keybinding_handler(self);

        if self.show_hit_counter.load(Ordering::Relaxed) {
            counter(self, ctx);
        }

        left_panel(self, ctx);

        match self.clone().open_page {
            Pages::List => list(self, ctx),
            Pages::Settings => configuration(self, ctx),
        }

        if self.show_hit_counter.load(Ordering::Relaxed) {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if !self.config.autosave {
            self.config.save();
            if let Some(cat) = &self.loaded_category {
                cat.save();
            }
        }
    }
}
