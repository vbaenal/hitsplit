use serde::{Deserialize, Serialize};

use crate::run::game::SmallGame;

#[derive(Deserialize)]
pub struct OptionalConfig {
    dark_mode: Option<bool>,
    next_split_as_reset: Option<bool>,
    autosave: Option<bool>,
    autosave_interval: Option<u64>,
    game_list: Option<Vec<SmallGame>>,
}

impl OptionalConfig {
    fn to_config(self) -> Config {
        Config {
            dark_mode: match self.dark_mode {
                None => true,
                Some(b) => b,
            },
            next_split_as_reset: match self.next_split_as_reset {
                None => true,
                Some(b) => b,
            },
            autosave: match self.autosave {
                None => true,
                Some(b) => b,
            },
            autosave_interval: match self.autosave_interval {
                None => 5,
                Some(u) => u,
            },
            game_list: match self.game_list {
                None => Vec::new(),
                Some(v) => v,
            },
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
    pub fn save(&mut self) {
        let config_str = serde_json::to_string(self).unwrap();
        let _ = std::fs::write("config/config.json", config_str);
    }

    pub fn load() -> Self {
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

        serde_json::from_str::<OptionalConfig>(config_json.as_str())
            .unwrap()
            .to_config()
    }
}
