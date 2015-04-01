use scene::{Scene, BoxedScene, SceneCommand};
use game_scene::GameScene;
use opengl_graphics::Gl;
use event::{RenderArgs, UpdateArgs};
use input::Button;
use graphics;

pub struct TitleScene;

impl TitleScene {
    pub fn new() -> BoxedScene {
        Box::new(TitleScene)
    }
}

impl Scene for TitleScene {

    fn render(&self, gl: &mut Gl, args: &RenderArgs) {
        graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
    }

    fn think(&mut self, args: &UpdateArgs) -> Option<SceneCommand> {
        println!("Hello, world!");
        Some(SceneCommand::SetScene(GameScene::new()))
    }
    
    fn press(&mut self, button: &Button) {}
    fn release(&mut self, button: &Button) {}
}
