use serde::{Deserialize, Serialize};

use crate::get_config_path;

use super::split::Split;

#[derive(Serialize, Deserialize, Clone, Default)]
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
        let config_path = get_config_path();
        let game_json: String =
            match std::fs::read_to_string(format!("{config_path}/categories/{uuid}.json")) {
                Err(_) => "".to_string(),
                Ok(f) => f,
            };

        serde_json::from_str(game_json.as_str()).unwrap()
    }

    pub fn save(&self) {
        let config_path = get_config_path();
        let category_str = serde_json::to_string(&self).unwrap();
        let _ = std::fs::write(
            format!("{config_path}/categories/{}.json", self.uuid),
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
