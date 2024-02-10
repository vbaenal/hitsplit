pub fn image_button(image_src: egui::ImageSource<'_>) -> egui::ImageButton<'_> {
    egui::widgets::ImageButton::new(
        egui::Image::new(image_src)
            .max_size(egui::Vec2::new(64.0, 64.0))
            .rounding(10.0),
    )
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
