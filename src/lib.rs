mod run;
mod settings;
mod ui;

use directories::ProjectDirs;
pub use ui::hitsplit::HitSplit;

pub fn get_config_path() -> String {
    let mut config_path: String = "config".to_owned();

    if let Some(proj_dirs) = ProjectDirs::from("", "", "HitSplit") {
        proj_dirs
            .config_dir()
            .to_str()
            .unwrap_or("config")
            .clone_into(&mut config_path);
    }

    config_path
}
