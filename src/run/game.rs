use serde::{Deserialize, Serialize};

use super::category::SmallCategory;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub uuid: String,
    pub name: String,
    pub categories: Vec<SmallCategory>,
}

impl Game {
    pub fn new(uuid: String, name: String) -> Self {
        Game {
            uuid,
            name,
            categories: Vec::new(),
        }
    }

    pub fn save(&self) {
        let game_str = serde_json::to_string(&self).unwrap();
        let _ = std::fs::write(format!("config/games/{}.json", self.uuid), game_str);
    }

    pub fn load(uuid: String) -> Self {
        let game_json: String = match std::fs::read_to_string(format!("config/games/{uuid}.json")) {
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
