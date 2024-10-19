use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{get_config_path, Error};

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

    pub fn save(&self) -> Result<(), Error> {
        let config_path = get_config_path();
        let game_str = match serde_json::to_string(&self) {
            Ok(game) => game,
            Err(e) => {
                return Err(Error::new(
                    format!(
                        "Could not serialize game {} with uuid {}",
                        self.name, self.uuid
                    ),
                    e.to_string(),
                ))
            }
        };
        match std::fs::write(format!("{config_path}/games/{}.json", self.uuid), game_str) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                format!(
                    "Could not save game with uuid {} on path \"{config_path}/games/{}.json\"",
                    self.uuid, self.uuid
                ),
                e.to_string(),
            )),
        }
    }

    pub fn load(uuid: String) -> Result<Self, Error> {
        let config_path = get_config_path();
        let game_json: String =
            match std::fs::read_to_string(format!("{config_path}/games/{uuid}.json")) {
                Err(_) => "".to_string(),
                Ok(f) => f,
            };

        match serde_json::from_str(game_json.as_str()) {
            Ok(category) => Ok(category),
            Err(e) => Err(Error::new(
                format!(
                    "Could not parse category json file: \"{config_path}/categories/{uuid}.json\""
                ),
                e.to_string(),
            )),
        }
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
