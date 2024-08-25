use std::fs::read_dir;

use egui::Key;
use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, HotKeyState,
};
use serde::{Deserialize, Serialize};

use crate::{
    get_config_path,
    run::manager::{
        add_hit, next_split, pause_chrono, prev_split, reset, set_pb, start_chrono, sub_hit,
    },
    HitSplit,
};

use super::key_to_code;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ShortcutAction {
    PrevSplit,
    NextSplit,
    AddHit,
    SubHit,
    Reset,
    SetPb,
    StartChrono,
    PauseChrono,
}

impl ShortcutAction {
    pub fn from_usize(u: usize) -> Option<Self> {
        match u {
            0 => Some(ShortcutAction::PrevSplit),
            1 => Some(ShortcutAction::NextSplit),
            2 => Some(ShortcutAction::AddHit),
            3 => Some(ShortcutAction::SubHit),
            4 => Some(ShortcutAction::Reset),
            5 => Some(ShortcutAction::SetPb),
            6 => Some(ShortcutAction::StartChrono),
            7 => Some(ShortcutAction::PauseChrono),
            _ => None,
        }
    }

    pub fn to_usize(self) -> usize {
        match self {
            ShortcutAction::PrevSplit => 0,
            ShortcutAction::NextSplit => 1,
            ShortcutAction::AddHit => 2,
            ShortcutAction::SubHit => 3,
            ShortcutAction::Reset => 4,
            ShortcutAction::SetPb => 5,
            ShortcutAction::StartChrono => 6,
            ShortcutAction::PauseChrono => 7,
        }
    }

    pub fn to_function(self) -> impl Fn(&mut HitSplit) {
        match self {
            ShortcutAction::PrevSplit => prev_split,
            ShortcutAction::NextSplit => next_split,
            ShortcutAction::AddHit => add_hit,
            ShortcutAction::SubHit => sub_hit,
            ShortcutAction::Reset => reset,
            ShortcutAction::SetPb => set_pb,
            ShortcutAction::StartChrono => start_chrono,
            ShortcutAction::PauseChrono => pause_chrono,
        }
    }

    pub fn change_shortcut(app: &mut HitSplit, action: &ShortcutAction, key: &Key) {
        //let shortcut = &mut app.shortcut.as_mut().unwrap().0;
        if let Some(shortcut) = app.shortcut.as_mut() {
            if !shortcut.0.contains(&key_to_code(key)) {
                if let Some(sc) = shortcut.0.get_mut(action.to_usize()) {
                    *sc = key_to_code(key);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shortcut(pub Vec<Code>);

impl Default for Shortcut {
    fn default() -> Self {
        Shortcut(vec![
            Code::Numpad8,
            Code::Numpad2,
            Code::Numpad7,
            Code::Numpad9,
            Code::Numpad5,
            Code::Numpad3,
            Code::Numpad4,
            Code::Numpad6,
        ])
    }
}

impl Shortcut {
    pub fn save(&self) {
        let config_path = get_config_path();
        let shortcuts_str = serde_json::to_string(self).unwrap();
        std::fs::write(format!("{config_path}/shortcuts.json"), shortcuts_str).unwrap();
    }

    pub fn load() -> Self {
        let config_path = get_config_path();
        if read_dir(&config_path).is_err() {
            let _ = std::fs::create_dir(&config_path);
        }

        let shortcuts_json: String =
            match std::fs::read_to_string(format!("{config_path}/shortcuts.json")) {
                Err(_) => {
                    let tmp: Shortcut = Default::default();
                    let shortcuts_str = serde_json::to_string(&tmp).unwrap();
                    std::fs::write(
                        format!("{config_path}/shortcuts.json"),
                        shortcuts_str.clone(),
                    )
                    .unwrap();
                    shortcuts_str
                }
                Ok(f) => f,
            };

        let shortcuts = serde_json::from_str::<Shortcut>(shortcuts_json.as_str()).unwrap();
        if shortcuts.0.len() < 8 {
            return Shortcut::default();
        }
        shortcuts
    }

    pub fn code_to_hotkey(code: Code) -> HotKey {
        HotKey::new(None, code)
    }

    fn code_to_id(code: Code) -> u32 {
        Self::code_to_hotkey(code).id()
    }
}

pub fn shortcut_handler(app: &mut HitSplit) {
    let receiver = GlobalHotKeyEvent::receiver();
    if let Ok(event) = receiver.try_recv() {
        if event.state == HotKeyState::Pressed {
            let shortcut = &app.shortcut.as_ref().unwrap().0;
            let index = shortcut
                .iter()
                .enumerate()
                .find(|(_, &c)| event.id == Shortcut::code_to_id(c))
                .unwrap()
                .0;
            ShortcutAction::from_usize(index).unwrap().to_function()(app);
        }
    }
}
