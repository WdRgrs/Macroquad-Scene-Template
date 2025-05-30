pub mod main;
pub mod manager;
pub mod template;

pub use main::MainScene;
pub use manager::SceneManager;
pub use template::TemplateScene;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum GameScene {
    Main,
    Template,
}

pub trait Scene {
    fn initialize() -> Self
    where
        Self: Sized;

    fn update(&mut self) -> Option<GameScene>;
    fn draw(&self);

    fn startup(&mut self);
    fn cleanup(&mut self);
}
