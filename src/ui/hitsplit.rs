use super::{
    counter::counter,
    panels::{bottom_panel, left_panel, list::list, settings::configuration, Pages},
    ChangeImage,
};
use crate::{
    run::{category::Category, chrono::Chronometer, game::Game},
    settings::{
        config::Config,
        shortcut::{shortcut_handler, Shortcut, ShortcutAction},
    },
    Error,
};
use eframe::{egui::Visuals, Storage};
use egui::Vec2;
use egui_file::FileDialog;
use global_hotkey::{hotkey::Code, GlobalHotKeyManager};
use std::{path::PathBuf, time::Duration};

pub struct HitSplit {
    pub config: Config,
    pub shortcut: Option<Shortcut>,
    pub num_splits_category: usize,
    pub open_page: Pages,
    pub add_game_name: String,
    pub add_game_open: bool,
    pub add_game_empty: bool,
    pub modify_game_open: bool,
    pub add_category_name: String,
    pub add_category_open: bool,
    pub add_category_empty: bool,
    pub modify_category_open: bool,
    pub delete_split: Option<usize>,
    pub add_split_under: Option<usize>,
    pub loaded_game: Option<Game>,
    pub loaded_category: Option<Category>,
    pub selected_split: usize,
    pub show_config: bool,
    pub hotkey_manager: Option<GlobalHotKeyManager>,
    pub capturing: Option<ShortcutAction>,
    pub opened_file: Option<PathBuf>,
    pub open_file_dialog: Option<FileDialog>,
    pub change_image: Option<ChangeImage>,
    pub chrono: Chronometer,
    pub error: Error,
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
            modify_game_open: self.modify_game_open,
            add_category_name: self.add_category_name.clone(),
            add_category_open: self.add_category_open,
            add_category_empty: self.add_category_empty,
            modify_category_open: self.modify_category_open,
            delete_split: self.delete_split,
            add_split_under: self.add_split_under,
            loaded_game: self.loaded_game.clone(),
            loaded_category: self.loaded_category.clone(),
            selected_split: self.selected_split,
            show_config: self.show_config,
            hotkey_manager: None,
            capturing: self.capturing,
            opened_file: self.opened_file.clone(),
            open_file_dialog: None,
            change_image: None,
            chrono: self.chrono,
            error: self.error.clone(),
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
            modify_game_open: false,
            add_category_name: "".to_string(),
            add_category_open: false,
            add_category_empty: false,
            modify_category_open: false,
            delete_split: None,
            add_split_under: None,
            loaded_game: None,
            loaded_category: None,
            selected_split: 0,
            show_config: true,
            hotkey_manager: None,
            capturing: None,
            opened_file: None,
            open_file_dialog: None,
            change_image: None,
            chrono: Chronometer::new(crate::run::chrono::ChronometerFormat::HHMMSSX),
            error: Error::None,
        }
    }
}

impl HitSplit {
    fn add_hotkey(&mut self, code: Code) {
        match self.hotkey_manager.as_ref() {
            Some(mngr) => {
                if let Err(e) = mngr.register(Shortcut::code_to_hotkey(code)) {
                    self.error =
                        Error::new(format!("Failed to register hotkey {}", code), e.to_string());
                }
            }
            None => {
                self.error = Error::new("Failed to get hotkey manager".to_string(), "".to_string())
            }
        };
    }

    fn manage_hotkeys(&mut self) {
        match GlobalHotKeyManager::new() {
            Ok(mngr) => self.hotkey_manager = Some(mngr),
            Err(_) => self.hotkey_manager = None,
        };
        if let Some(sc) = self.shortcut.clone() {
            sc.0.iter().for_each(|&c| self.add_hotkey(c));
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: HitSplit = HitSplit {
            config: Config::load().unwrap_or_default(),
            shortcut: Some(Shortcut::load().unwrap_or_default()),
            ..Default::default()
        };
        app.chrono.set_format(&app.config.chrono_format);
        app.manage_hotkeys();

        cc.egui_ctx.set_visuals(if app.config.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        });

        egui_extras::install_image_loaders(&cc.egui_ctx);
        app
    }

    fn save_files(&mut self) {
        if let Err(e) = self.config.save() {
            self.error = e;
        }
        if let Some(s) = &self.shortcut {
            if let Err(e) = s.save() {
                self.error = e;
            }
        }
        if let Some(g) = &self.loaded_game {
            if let Err(e) = g.save() {
                self.error = e;
            }
        }
        if let Some(c) = &self.loaded_category {
            if let Err(e) = c.save() {
                self.error = e;
            }
        }
    }
}

impl eframe::App for HitSplit {
    fn save(&mut self, _storage: &mut dyn Storage) {
        if self.config.autosave {
            self.save_files();
        }
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        Duration::new(self.config.autosave_interval, 0)
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.config.dark_mode = ctx.style().visuals.dark_mode;
        if self.config.always_on_top {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                egui::WindowLevel::AlwaysOnTop,
            ));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                egui::WindowLevel::Normal,
            ));
        }

        if self.capturing.is_none() {
            match shortcut_handler(self) {
                Ok(_) => (),
                Err(e) => self.error = e,
            };
        }

        if let Some(category) = self.loaded_category.as_mut() {
            if let Some(split) = category.splits.get_mut(self.selected_split) {
                match self.chrono.get_time() {
                    Ok(rt) => split.real_time = rt,
                    Err(e) => self.error = e,
                };
            }
        }

        counter(self, ctx);

        if self.show_config {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("hitsplit_manager"),
                egui::ViewportBuilder::default()
                    .with_title("HitSplit Manager")
                    .with_resizable(true)
                    .with_inner_size(Vec2::new(800.0, 800.0))
                    .with_min_inner_size(Vec2::new(650.0, 600.0)),
                move |ctx, _class| {
                    if let Some(sa) = &self.capturing.clone() {
                        if let Some(key) = ctx.input(|i| i.keys_down.clone().into_iter().last()) {
                            ShortcutAction::change_shortcut(self, sa, &key);
                            self.manage_hotkeys();
                            self.capturing = None;
                        }
                    }

                    left_panel(self, ctx);

                    match self.open_page {
                        Pages::List => list(self, ctx),
                        Pages::Settings => configuration(self, ctx),
                    }
                    bottom_panel(self, ctx);

                    if ctx.input(|i| i.raw.viewport().close_requested()) {
                        self.show_config = false;
                    }
                },
            );
        }

        // 60 fps
        ctx.request_repaint_after(Duration::from_micros(16666));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_files();
    }
}
