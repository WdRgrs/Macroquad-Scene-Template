use macroquad::prelude::*;

pub fn draw_title(text: &str, key: &str) {
    // Center the text
    let font_size = 30.;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let x = (screen_width() - text_size.width) / 2.0;

    draw_text(text, x, screen_height() * 0.4, font_size, BEIGE);

    let destination = if matches!(key, "A") {"About"} else {"Template"};
    let text = format!("Press {key} to transition to the {destination} scene");
    let font_size = 20.;
    let text_size = measure_text(&text, None, font_size as u16, 1.0);
    let x = (screen_width() - text_size.width) / 2.0;

    draw_text(&text, x, screen_height() * 0.6, font_size, BEIGE);
}
