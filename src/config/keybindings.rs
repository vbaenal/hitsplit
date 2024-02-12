use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, HotKeyState,
};

use crate::{
    run::manager::{add_hit, next_split, prev_split, reset, set_pb, sub_hit},
    HitSplit,
};

#[derive(Clone)]
pub struct Keybindings {
    pub prev_split: HotKey,
    pub next_split: HotKey,
    pub add_hit: HotKey,
    pub sub_hit: HotKey,
    pub reset: HotKey,
    pub set_pb: HotKey,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            prev_split: HotKey::new(None, Code::Numpad8),
            next_split: HotKey::new(None, Code::Numpad2),
            add_hit: HotKey::new(None, Code::Numpad7),
            sub_hit: HotKey::new(None, Code::Numpad9),
            reset: HotKey::new(None, Code::Numpad5),
            set_pb: HotKey::new(None, Code::Numpad3),
        }
    }
}

pub fn keybinding_handler(app: &mut HitSplit) {
    let receiver = GlobalHotKeyEvent::receiver();
    if let Ok(event) = receiver.try_recv() {
        if event.state == HotKeyState::Pressed {
            if event.id == app.keybinding.clone().unwrap().prev_split.id() {
                prev_split(app);
            } else if event.id == app.keybinding.clone().unwrap().next_split.id() {
                next_split(app);
            } else if event.id == app.keybinding.clone().unwrap().sub_hit.id() {
                sub_hit(app);
            } else if event.id == app.keybinding.clone().unwrap().add_hit.id() {
                add_hit(app);
            } else if event.id == app.keybinding.clone().unwrap().reset.id() {
                reset(app);
            } else if event.id == app.keybinding.clone().unwrap().set_pb.id() {
                set_pb(app);
            }
        }
    }
}
