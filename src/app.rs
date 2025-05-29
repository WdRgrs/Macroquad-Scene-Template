use crate::scenes::SceneManager;
use macroquad::prelude::*;

pub struct App {
    scene_manager: SceneManager,
}

impl App {
    pub async fn new() -> Self {
        let app = Self {
            scene_manager: SceneManager::new().await,
        };

        app
    }

    pub fn update(&mut self) {
        self.scene_manager.update();
    }

    pub fn draw(&self) {
        self.scene_manager.draw();
    }

    pub async fn run(&mut self) {
        loop {
            self.update();

            self.draw();

            next_frame().await;
        }
    }
}
