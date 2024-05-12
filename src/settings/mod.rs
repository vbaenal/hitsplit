use egui::Key;
use global_hotkey::hotkey::Code;

pub mod config;
pub mod shortcut;

pub fn key_to_code(key: &Key) -> Code {
    match key {
        Key::ArrowDown => Code::ArrowDown,
        Key::ArrowLeft => Code::ArrowLeft,
        Key::ArrowRight => Code::ArrowRight,
        Key::ArrowUp => Code::ArrowUp,
        Key::Escape => Code::Escape,
        Key::Tab => Code::Tab,
        Key::Backspace => Code::Backspace,
        Key::Enter => Code::Enter,
        Key::Space => Code::Space,
        Key::Insert => Code::Insert,
        Key::Delete => Code::Delete,
        Key::Home => Code::Home,
        Key::End => Code::End,
        Key::PageUp => Code::PageUp,
        Key::PageDown => Code::PageDown,
        Key::Copy => Code::Copy,
        Key::Cut => Code::Cut,
        Key::Paste => Code::Paste,
        Key::Colon => Code::Semicolon,
        Key::Comma => Code::Comma,
        Key::Backslash => Code::Backslash,
        Key::Slash => Code::Slash,
        Key::Pipe => Code::Backslash,
        Key::Questionmark => Code::Slash,
        Key::OpenBracket => Code::BracketLeft,
        Key::CloseBracket => Code::BracketRight,
        Key::Backtick => Code::Backquote,
        Key::Minus => Code::Minus,
        Key::Period => Code::Period,
        Key::Plus => Code::Equal,
        Key::Equals => Code::Equal,
        Key::Semicolon => Code::Semicolon,
        Key::Num0 => Code::Numpad0,
        Key::Num1 => Code::Numpad1,
        Key::Num2 => Code::Numpad2,
        Key::Num3 => Code::Numpad3,
        Key::Num4 => Code::Numpad4,
        Key::Num5 => Code::Numpad5,
        Key::Num6 => Code::Numpad6,
        Key::Num7 => Code::Numpad7,
        Key::Num8 => Code::Numpad8,
        Key::Num9 => Code::Numpad9,
        Key::A => Code::KeyA,
        Key::B => Code::KeyB,
        Key::C => Code::KeyC,
        Key::D => Code::KeyD,
        Key::E => Code::KeyE,
        Key::F => Code::KeyF,
        Key::G => Code::KeyG,
        Key::H => Code::KeyH,
        Key::I => Code::KeyI,
        Key::J => Code::KeyJ,
        Key::K => Code::KeyK,
        Key::L => Code::KeyL,
        Key::M => Code::KeyM,
        Key::N => Code::KeyN,
        Key::O => Code::KeyO,
        Key::P => Code::KeyP,
        Key::Q => Code::KeyQ,
        Key::R => Code::KeyR,
        Key::S => Code::KeyS,
        Key::T => Code::KeyT,
        Key::U => Code::KeyU,
        Key::V => Code::KeyV,
        Key::W => Code::KeyW,
        Key::X => Code::KeyX,
        Key::Y => Code::KeyY,
        Key::Z => Code::KeyZ,
        Key::F1 => Code::F1,
        Key::F2 => Code::F2,
        Key::F3 => Code::F3,
        Key::F4 => Code::F4,
        Key::F5 => Code::F5,
        Key::F6 => Code::F6,
        Key::F7 => Code::F7,
        Key::F8 => Code::F8,
        Key::F9 => Code::F9,
        Key::F10 => Code::F10,
        Key::F11 => Code::F11,
        Key::F12 => Code::F12,
        Key::F13 => Code::F13,
        Key::F14 => Code::F14,
        Key::F15 => Code::F15,
        Key::F16 => Code::F16,
        Key::F17 => Code::F17,
        Key::F18 => Code::F18,
        Key::F19 => Code::F19,
        Key::F20 => Code::F20,
        Key::F21 => Code::F21,
        Key::F22 => Code::F22,
        Key::F23 => Code::F23,
        Key::F24 => Code::F24,
        Key::F25 => Code::F25,
        Key::F26 => Code::F26,
        Key::F27 => Code::F27,
        Key::F28 => Code::F28,
        Key::F29 => Code::F29,
        Key::F30 => Code::F30,
        Key::F31 => Code::F31,
        Key::F32 => Code::F32,
        Key::F33 => Code::F33,
        Key::F34 => Code::F34,
        Key::F35 => Code::F35,
    }
}