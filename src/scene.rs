use opengl_graphics::Gl;
use event::RenderArgs;
use input::Button;

pub trait Scene {
    fn render(&self, gl: &mut Gl, args: &RenderArgs);
    fn think(&mut self) -> Option<SceneCommand>;
    fn input(&mut self, button: &Button);
}

pub type BoxedScene = Box<Scene + 'static>;

pub enum SceneCommand {
    SetScene(BoxedScene),
    PushScene(BoxedScene),
    PopScene,
    Clear
}
