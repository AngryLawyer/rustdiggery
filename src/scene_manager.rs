use scene::{BoxedScene, SceneCommand};
use piston_window::PistonWindow;
//use opengl_graphics::GlGraphics;
//use event::{RenderArgs, UpdateArgs};
//use input::Button;

pub struct SceneManager {
    scene_stack: Vec<BoxedScene>
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scene_stack: vec![]
        }
    }

    pub fn scene_count(&self) -> usize {
        self.scene_stack.len()
    }

    pub fn set_scene(&mut self, scene: BoxedScene) {
        self.scene_stack.pop();
        self.scene_stack.push(scene);
    }

    pub fn push_scene(&mut self, scene: BoxedScene) {
        self.scene_stack.push(scene)
    }

    pub fn pop_scene(&mut self) -> Option<BoxedScene> {
        self.scene_stack.pop()
    }

    pub fn clear(&mut self) {
        self.scene_stack.clear()
    }

    pub fn handle_event(&mut self, e: &PistonWindow) {
        match self.scene_stack.pop() {
            Some(mut scene) => {
                let result = scene.handle_event(e);
                self.scene_stack.push(scene);
                self.handle_scene_command(result);
            },
            None => ()
        }
    }

    fn handle_scene_command(&mut self, command: Option<SceneCommand>) {
        match command {
            Some(SceneCommand::SetScene(scene)) => {
                self.set_scene(scene);
            },
            Some(SceneCommand::PushScene(scene)) => {
                self.push_scene(scene);
            },
            Some(SceneCommand::PopScene) => {
                self.pop_scene();
            },
            Some(SceneCommand::Clear) => {
                self.clear();
            },
            _ => ()
        }
    }
}
