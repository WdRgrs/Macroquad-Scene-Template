use crate::scenes::GameScene;
use crate::scenes::Scene;
use crate::ui::draw_title;
use macroquad::prelude::*;

pub struct TemplateScene {
    next_scene: Option<GameScene>,
}

impl Scene for TemplateScene {
    fn initialize() -> Self {
        let scene = Self { next_scene: None };

        scene
    }
    
    fn update(&mut self) -> Option<GameScene> {
        if is_key_pressed(KeyCode::A) {
            self.next_scene = Some(GameScene::Main)
        }

        self.next_scene.take()
    }

    fn draw(&self) {
        clear_background(DARKPURPLE);
        draw_title("TEMPLATE PAGE", "A");
    }

    fn startup(&mut self) {
        println!("- - - - template scene is starting - - - -");
    }

    fn cleanup(&mut self) {
        println!("-- template scene is getting cleaned up --");
    }
}
