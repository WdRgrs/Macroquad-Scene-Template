use macroquad::prelude::*;
use std::collections::HashMap;

use super::{AboutScene, GameScene, Scene, TemplateScene};

pub struct SceneManager {
    scenes: HashMap<GameScene, Box<dyn Scene>>,
    current_scene: GameScene,
    next_scene: Option<GameScene>,
}

impl SceneManager {
    pub async fn new() -> Self {
        let scenes: HashMap<GameScene, Box<dyn Scene + 'static>> = HashMap::new();

        // Initialize with About scene
        let current_scene = GameScene::About;

        let mut manager = Self {
            scenes,
            current_scene,
            next_scene: None,
        };

        manager.initalize_scenes();

        manager
    }

    fn initalize_scenes(&mut self) {
        self.scenes.insert(
            GameScene::Template,
            Box::new(TemplateScene::initialize()) as Box<dyn Scene>,
        );
        self.scenes.insert(
            GameScene::About,
            Box::new(AboutScene::initialize()) as Box<dyn Scene>,
        );
    }

    pub fn update(&mut self) {
        // check for scene change - returned from scene's update fn
        if let Some(next_scene) = self.next_scene.take() {
            self.transition_to(next_scene);
        }

        if let Some(scene) = self.scenes.get_mut(&self.current_scene) {
            // Update the current scene
            if let Some(new_scene) = scene.update() {
                self.next_scene = Some(new_scene);
            }
        }
    }

    pub fn draw(&self) {
        if let Some(scene) = self.scenes.get(&self.current_scene) {
            scene.draw();
        }
    }

    fn transition_to(&mut self, new_scene: GameScene) {
        // Don't change if it's the same scene
        if self.current_scene == new_scene {
            return;
        }

        self.cleanup_old_scene();

        self.start_new_scene(new_scene);
    }

    fn start_new_scene(&mut self, new_scene: GameScene) {
        // Change to new scene and run startup functions
        if let Some(scene) = self.scenes.get_mut(&new_scene) {
            self.current_scene = new_scene;

            scene.startup();
        } else {
            // ! Handle not finidng new scene - maybe redirect to error page or main menu
        }
    }

    fn cleanup_old_scene(&mut self) {
        if let Some(scene) = self.scenes.get_mut(&self.current_scene) {
            scene.cleanup();
        }
    }
}
