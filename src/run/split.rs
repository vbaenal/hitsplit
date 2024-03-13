use std::{path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Split {
    pub uuid: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub name: String,
    pub hits: u16,
    pub pb: u16,
    pub real_time: Duration,
}

impl Split {
    pub fn new(uuid: Option<String>) -> Self {
        Self {
            uuid,
            ..Default::default()
        }
    }
}
