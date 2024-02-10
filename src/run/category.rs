use serde::{Deserialize, Serialize};

use super::split::Split;

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub uuid: String,
    pub name: String,
    pub splits: Vec<Split>,
}

impl Category {
    pub fn new(uuid: String, name: String) -> Self {
        Category {
            uuid: uuid,
            name: name,
            splits: Vec::new(),
        }
    }

    pub fn load(uuid: String) -> Self {
        let game_json: String =
            match std::fs::read_to_string(format!("config/categories/{uuid}.json")) {
                Err(_) => "".to_string(),
                Ok(f) => f,
            };

        serde_json::from_str(game_json.as_str()).unwrap()
    }

    pub fn save(&self) {
        let category_str = serde_json::to_string(&self).unwrap();
        let _ = std::fs::write(
            format!("config/categories/{}.json", self.uuid),
            category_str,
        );
    }

    pub fn save_splits(&mut self, splits: Vec<Split>) {
        self.splits = splits;
        self.save();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SmallCategory {
    pub uuid: String,
    pub name: String,
}
