use scene::{BoxedScene, SceneCommand};
use opengl_graphics::Gl;
use event::{RenderArgs};
use input::Button;

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

    pub fn render(&self, gl: &mut Gl, args: &RenderArgs) {
        if self.scene_stack.len() > 0 {
            self.scene_stack[self.scene_stack.len() - 1].render(gl, args);
        }
    }

    pub fn think(&mut self)  {
        match self.scene_stack.pop() {
            Some(mut scene) => {
                let result = scene.think();
                self.scene_stack.push(scene);
                self.handle_scene_command(result);
            },
            None => ()
        }
    }

    pub fn input(&mut self, args: &Button) {
        match self.scene_stack.pop() {
            Some(mut scene) => {
                scene.input(args);
                self.scene_stack.push(scene);
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
