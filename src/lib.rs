mod run;
mod settings;
mod ui;

use std::path::PathBuf;

use directories::ProjectDirs;
use egui_file::FileDialog;
pub use ui::hitsplit::HitSplit;

fn get_pictures_path() -> PathBuf {
    if let Some(d) = directories::UserDirs::new() {
        if let Some(p) = d.picture_dir() {
            p.to_path_buf()
        } else {
            d.home_dir().to_path_buf()
        }
    } else {
        PathBuf::default()
    }
}

pub fn get_config_path() -> String {
    let mut config_path: String = "config".to_owned();

    if let Some(proj_dirs) = ProjectDirs::from("", "", "HitSplit") {
        proj_dirs
            .config_dir()
            .to_str()
            .unwrap_or("config")
            .clone_into(&mut config_path);
    }

    if cfg!(debug_assertions) {
        config_path += "/debug";
    }

    config_path
}

pub fn get_file_dialog(path: Option<PathBuf>) -> FileDialog {
    if let Some(p) = path {
        if p == PathBuf::default() {
            FileDialog::open_file(Some(get_pictures_path()))
        } else {
            FileDialog::open_file(Some(p))
        }
    } else {
        FileDialog::open_file(Some(get_pictures_path()))
    }
}
