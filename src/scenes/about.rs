use crate::scenes::GameScene;
use crate::scenes::Scene;
use crate::ui::draw_title;
use macroquad::prelude::*;

pub struct MainScene {
    next_scene: Option<GameScene>,
}

impl Scene for MainScene {
    fn initialize() -> Self {
        let scene = Self { next_scene: None };

        scene
    }
    
    fn update(&mut self) -> Option<GameScene> {
        if is_key_pressed(KeyCode::T) {
            self.next_scene = Some(GameScene::Template)
        }

        self.next_scene.take()
    }

    fn draw(&self) {
        draw_title("Main Scene", "T")
    }

    fn startup(&mut self) {
        println!("Main scene started!");
    }

    fn cleanup(&mut self) {
        println!("Main scene cleaned up!");
    }
}
