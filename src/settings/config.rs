use std::fs::read_dir;

use egui::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    get_config_path,
    run::{chrono::ChronometerFormat, game::SmallGame},
};

use super::columns::ColumnVec;

#[derive(Deserialize)]
pub struct OptionalConfig {
    dark_mode: Option<bool>,
    next_split_as_reset: Option<bool>,
    autosave: Option<bool>,
    autosave_interval: Option<u64>,
    game_list: Option<Vec<SmallGame>>,
    font_size: Option<f32>,
    limit_splits_shown: Option<bool>,
    num_splits_counter: Option<usize>,
    counter_size: Option<Vec2>,
    columns: Option<ColumnVec>,
    chrono_format: Option<ChronometerFormat>,
    game_image_height: Option<f32>,
    category_image_height: Option<f32>,
    background_color: Option<[u8; 3]>,
    background_transparency: Option<u8>,
    text_color_default: Option<[u8; 3]>,
    text_color_nohit: Option<[u8; 3]>,
    text_color_better: Option<[u8; 3]>,
    text_color_worse: Option<[u8; 3]>,
    always_on_top: Option<bool>,
}

impl OptionalConfig {
    fn to_config(&self) -> Config {
        Config {
            dark_mode: self.dark_mode.unwrap_or(true),
            next_split_as_reset: self.next_split_as_reset.unwrap_or(true),
            autosave: self.autosave.unwrap_or(true),
            autosave_interval: self.autosave_interval.unwrap_or(60),
            game_list: match &self.game_list {
                None => Vec::new(),
                Some(v) => v.to_vec(),
            },
            font_size: self.font_size.unwrap_or(14.0),
            limit_splits_shown: self.limit_splits_shown.unwrap_or(false),
            num_splits_counter: self.num_splits_counter.unwrap_or(10),
            counter_size: self.counter_size.unwrap_or([280.0, 600.0].into()),
            columns: self.columns.clone().unwrap_or_default(),
            chrono_format: self.chrono_format.unwrap_or(ChronometerFormat::HHMMSSX),
            game_image_height: self.game_image_height.unwrap_or(46.),
            category_image_height: self.category_image_height.unwrap_or(40.),
            background_color: self.background_color.unwrap_or([28, 28, 28]),
            background_transparency: self.background_transparency.unwrap_or(255),
            text_color_default: self.text_color_default.unwrap_or([240, 240, 240]),
            text_color_nohit: self.text_color_nohit.unwrap_or([8, 250, 8]),
            text_color_better: self.text_color_better.unwrap_or([250, 250, 8]),
            text_color_worse: self.text_color_worse.unwrap_or([250, 8, 8]),
            always_on_top: self.always_on_top.unwrap_or(true),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Config {
    pub dark_mode: bool,
    pub next_split_as_reset: bool,
    pub autosave: bool,
    pub autosave_interval: u64,
    pub game_list: Vec<SmallGame>,
    pub font_size: f32,
    pub limit_splits_shown: bool,
    pub num_splits_counter: usize,
    pub counter_size: Vec2,
    pub columns: ColumnVec,
    pub chrono_format: ChronometerFormat,
    pub game_image_height: f32,
    pub category_image_height: f32,
    pub background_color: [u8; 3],
    pub background_transparency: u8,
    pub text_color_default: [u8; 3],
    pub text_color_nohit: [u8; 3],
    pub text_color_better: [u8; 3],
    pub text_color_worse: [u8; 3],
    pub always_on_top: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dark_mode: true,
            next_split_as_reset: true,
            autosave: true,
            autosave_interval: 60,
            game_list: Vec::new(),
            font_size: 14.0,
            limit_splits_shown: false,
            num_splits_counter: 0,
            counter_size: [280.0, 600.0].into(),
            columns: ColumnVec::default(),
            chrono_format: ChronometerFormat::HHMMSSX,
            game_image_height: 46.,
            category_image_height: 40.,
            background_color: [28, 28, 28],
            background_transparency: 255,
            text_color_default: [240, 240, 240],
            text_color_nohit: [8, 250, 8],
            text_color_better: [250, 250, 8],
            text_color_worse: [250, 8, 8],
            always_on_top: true,
        }
    }
}

impl Config {
    pub fn save(&mut self) {
        let config_path = get_config_path();
        let config_str = serde_json::to_string(self).unwrap();
        let _ = std::fs::write(format!("{config_path}/config.json"), config_str);
    }

    pub fn load() -> Self {
        let config_path = get_config_path();

        if read_dir(&config_path).is_err() {
            let _ = std::fs::create_dir(&config_path);
        }

        if read_dir(format!("{config_path}/games")).is_err() {
            let _ = std::fs::create_dir(format!("{config_path}/games"));
        }

        if read_dir(format!("{config_path}/categories")).is_err() {
            let _ = std::fs::create_dir(format!("{config_path}/categories"));
        }

        let config_json: String =
            match std::fs::read_to_string(format!("{config_path}/config.json")) {
                Err(_) => {
                    let tmp: Config = Default::default();
                    let config_str = serde_json::to_string(&tmp).unwrap();
                    let _ =
                        std::fs::write(format!("{config_path}/config.json"), config_str.clone());
                    config_str
                }
                Ok(f) => f,
            };

        serde_json::from_str::<OptionalConfig>(config_json.as_str())
            .unwrap()
            .to_config()
    }
}
