use macroquad::prelude::*;

mod app;
mod scenes;
mod ui;

use app::App;

fn window_conf() -> Conf {
    Conf {
        window_title: "{{project-name}}".into(),
        window_width: 960,
        window_height: 540,
        // optional
        fullscreen: false,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // init game & run
    let mut app = App::new().await;

    app.run().await;
}
