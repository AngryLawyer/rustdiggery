use opengl_graphics::GlGraphics;
use event::{RenderArgs, UpdateArgs};
use input::Button;

pub trait Scene {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs);
    fn think(&mut self, args: &UpdateArgs) -> Option<SceneCommand>;
    fn press(&mut self, button: &Button);
    fn release(&mut self, button: &Button);
}

pub type BoxedScene = Box<Scene + 'static>;

pub enum SceneCommand {
    SetScene(BoxedScene),
    PushScene(BoxedScene),
    PopScene,
    Clear
}
