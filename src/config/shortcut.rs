use egui::Key;
use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, HotKeyState,
};
use serde::{Deserialize, Serialize};

use crate::{
    run::manager::{add_hit, next_split, prev_split, reset, set_pb, sub_hit},
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
        }
    }

    pub fn to_function(self) -> impl Fn(&mut HitSplit) -> () {
        match self {
            ShortcutAction::PrevSplit => prev_split,
            ShortcutAction::NextSplit => next_split,
            ShortcutAction::AddHit => add_hit,
            ShortcutAction::SubHit => sub_hit,
            ShortcutAction::Reset => reset,
            ShortcutAction::SetPb => set_pb,
        }
    }

    pub fn change_shortcut(app: &mut HitSplit, action: &ShortcutAction, key: Key) {
        let shortcut: &mut [Code; 6] = &mut app.shortcut.as_mut().unwrap().0;
        if !shortcut.contains(&key_to_code(&key)) {
            shortcut[action.to_usize()] = key_to_code(&key);
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shortcut(pub [Code; 6]);

impl Default for Shortcut {
    fn default() -> Self {
        Shortcut([
            Code::Numpad8,
            Code::Numpad2,
            Code::Numpad7,
            Code::Numpad9,
            Code::Numpad5,
            Code::Numpad3,
        ])
    }
}

impl Shortcut {
    pub fn save(&self) {
        let config_str = serde_json::to_string(self).unwrap();
        let _ = std::fs::write("config/shortcuts.json", config_str);
    }

    pub fn load() -> Self {
        match std::fs::read_dir("config") {
            Err(_) => {
                let _ = std::fs::create_dir("config");
                true
            }
            Ok(_) => true,
        };

        let shortcuts_json: String = match std::fs::read_to_string("config/shortcuts.json") {
            Err(_) => {
                let tmp: Shortcut = Default::default();
                let config_str = serde_json::to_string(&tmp).unwrap();
                let _ = std::fs::write("config/config.json", config_str.clone());
                config_str
            }
            Ok(f) => f,
        };

        serde_json::from_str::<Shortcut>(shortcuts_json.as_str()).unwrap()
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
            let shortcut: [Code; 6] = app.shortcut.as_ref().unwrap().0;
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
