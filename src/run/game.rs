use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::get_config_path;

use super::category::SmallCategory;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Game {
    pub uuid: String,
    pub name: String,
    pub icon_path: Option<PathBuf>,
    pub categories: Vec<SmallCategory>,
}

impl Game {
    pub fn new(uuid: String, name: String) -> Self {
        Game {
            uuid,
            name,
            ..Default::default()
        }
    }

    pub fn change_name(&mut self, new_name: &str) {
        new_name.clone_into(&mut self.name)
    }

    pub fn save(&self) {
        let config_path = get_config_path();
        let game_str = serde_json::to_string(&self).unwrap();
        let _ = std::fs::write(format!("{config_path}/games/{}.json", self.uuid), game_str);
    }

    pub fn load(uuid: String) -> Self {
        let config_path = get_config_path();
        let game_json: String =
            match std::fs::read_to_string(format!("{config_path}/games/{uuid}.json")) {
                Err(_) => "".to_string(),
                Ok(f) => f,
            };

        serde_json::from_str(game_json.as_str()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SmallGame {
    pub uuid: String,
    pub name: String,
}

impl SmallGame {
    pub fn change_name(&mut self, new_name: &str) {
        new_name.clone_into(&mut self.name)
    }
}
