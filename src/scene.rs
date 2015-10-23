use piston_window::PistonWindow;
//use opengl_graphics::GlGraphics;
//use event::{RenderArgs, UpdateArgs};
//use input::Button;

pub trait Scene {
    fn handle_event(&mut self, e: &PistonWindow) -> Option<SceneCommand>;
}

pub type BoxedScene = Box<Scene + 'static>;

pub enum SceneCommand {
    SetScene(BoxedScene),
    PushScene(BoxedScene),
    PopScene,
    Clear
}
