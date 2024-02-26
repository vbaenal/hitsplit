use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Split {
    pub icon_path: String,
    pub name: String,
    pub hits: u16,
    pub pb: u16,
    pub real_time: Duration,
}

impl Default for Split {
    fn default() -> Self {
        Self {
            icon_path: "".to_string(),
            name: "".to_string(),
            hits: 0,
            pb: 0,
            real_time: Duration::default(),
        }
    }
}
