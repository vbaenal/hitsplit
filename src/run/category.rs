use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{get_config_path, Error};

use super::split::Split;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Category {
    pub uuid: String,
    pub name: String,
    pub icon_path: Option<PathBuf>,
    pub splits: Vec<Split>,
}

impl Category {
    pub fn new(uuid: String, name: String) -> Self {
        Category {
            uuid,
            name,
            ..Default::default()
        }
    }

    pub fn load(uuid: String) -> Result<Self, Error> {
        let config_path = get_config_path();
        let category_json: String =
            match std::fs::read_to_string(format!("{config_path}/categories/{uuid}.json")) {
                Err(e) => return Err(Error::new(format!("Could not load category with uuid {uuid}. File not found: \"{config_path}/categories/{uuid}.json\""), e.to_string())),
                Ok(f) => f,
            };

        match serde_json::from_str(category_json.as_str()) {
            Ok(category) => Ok(category),
            Err(e) => Err(Error::new(
                format!(
                    "Could not parse category json file: \"{config_path}/categories/{uuid}.json\""
                ),
                e.to_string(),
            )),
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let config_path = get_config_path();
        let category_str = match serde_json::to_string(&self) {
            Ok(category) => category,
            Err(e) => {
                return Err(Error::new(
                    format!(
                        "Could not serialize category {} with uuid {}",
                        self.name, self.uuid
                    ),
                    e.to_string(),
                ))
            }
        };
        match std::fs::write(
            format!("{config_path}/categories/{}.json", self.uuid),
            category_str,
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(Error::new(
                    format!(
                        "Could not save category {0} with uuid {1} on path \"{config_path}/categories/{1}.json\"",
                        self.name, self.uuid
                    ),
                    e.to_string(),
                ))
            }
        }
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
