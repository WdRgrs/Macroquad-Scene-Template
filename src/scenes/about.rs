use macroquad::prelude::*;
use crate::scenes::GameScene;
use crate::scenes::Scene;
use crate::ui::draw_title;

pub struct AboutScene {
    next_scene: Option<GameScene>
}

impl Scene for AboutScene {
    fn initialize() -> Self {
        let scene = Self {
          next_scene: None
        };

        scene
    }
    fn update(&mut self) -> Option<GameScene> {
        if is_key_pressed(KeyCode::T) {
            self.next_scene = Some(GameScene::Template)
        }

        self.next_scene.take()
    }

    fn draw(&self) {
        draw_title("About Scene", "T")
    }

    fn startup(&mut self) {
        println!("About scene started!");
    }
    fn cleanup(&mut self) {
        println!("About scene cleaned up!");
    }
}
