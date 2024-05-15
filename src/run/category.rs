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
            uuid,
            name,
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

    pub fn change_name(&mut self, new_name: &str) {
        new_name.clone_into(&mut self.name)
    }

    pub fn clear_icon_path(&mut self) {
        self.splits
            .iter_mut()
            .for_each(|split| split.clear_icon_path());
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SmallCategory {
    pub uuid: String,
    pub name: String,
}

impl SmallCategory {
    pub fn change_name(&mut self, new_name: &str) {
        new_name.clone_into(&mut self.name)
    }
}
