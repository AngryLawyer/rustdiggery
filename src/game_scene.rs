use scene::{Scene, BoxedScene, SceneCommand};
use opengl_graphics::Gl;
use event::RenderArgs;
use input::Button;
use graphics;

#[derive(Clone)]
enum CellState {
    Empty,
    Dirt,
    Stone
}

struct World {
    cells: Vec<CellState>,
    width: u32,
    height: u32
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World {
            cells: vec![CellState::Empty; (width * height) as usize],
            width: width,
            height: height
        }
    }
}

pub struct GameScene {
    quit: bool
}

impl GameScene {
    pub fn new() -> BoxedScene {
        Box::new(GameScene { 
            quit: false,
        })
    }
}

impl Scene for GameScene {
    fn render(&self, gl: &mut Gl, args: &RenderArgs) {
        gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
        });
    }

    fn think(&mut self) -> Option<SceneCommand> {

        if (self.quit) {
            Some(SceneCommand::PopScene)
        } else {
            None
        }
    }

    fn input(&mut self, button: &Button) {
        self.quit = true;
    }
}
