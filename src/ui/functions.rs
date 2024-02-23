use crate::{config::shortcut::ShortcutAction, HitSplit};

pub fn image_button(image_src: egui::ImageSource<'_>) -> egui::ImageButton<'_> {
    egui::widgets::ImageButton::new(
        egui::Image::new(image_src)
            .max_size(egui::Vec2::new(64.0, 64.0))
            .rounding(10.0),
    )
}

pub fn shortcut_button(app: &mut HitSplit, ui: &mut egui::Ui, action: &ShortcutAction) {
    let capturing: bool = app.capturing.as_ref().is_some_and(|c| c == action);

    let btn_label = if capturing {
        "Press a key to change shortcut. Click again to cancel.".to_string()
    } else {
        app.shortcut
            .as_ref()
            .unwrap()
            .0
            .get(action.to_usize())
            .unwrap()
            .to_string()
    };
    if ui.button(btn_label).clicked() {
        if capturing {
            app.capturing = None;
        } else {
            app.capturing = Some(*action);
        }
    }
}

pub fn integer_edit_field_u64(ui: &mut egui::Ui, value: &mut u64, width: f32) -> egui::Response {
    let mut tmp_value = format!("{}", value);
    let res = ui.add(egui::TextEdit::singleline(&mut tmp_value).desired_width(width));
    if tmp_value == "" {
        tmp_value = "0".to_string();
    }
    if let Ok(result) = tmp_value.parse() {
        *value = result;
    }
    res
}

pub fn integer_edit_field_u16(ui: &mut egui::Ui, value: &mut u16) -> egui::Response {
    let mut tmp_value = format!("{}", value);
    let res = ui.add(egui::TextEdit::singleline(&mut tmp_value).desired_width(24.0));
    if tmp_value == "" {
        tmp_value = "0".to_string();
    }
    if let Ok(result) = tmp_value.parse() {
        *value = result;
    }
    res
}
