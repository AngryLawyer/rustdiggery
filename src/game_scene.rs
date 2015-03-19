use scene::{Scene, BoxedScene, SceneCommand};
use opengl_graphics::Gl;
use event::RenderArgs;
use input::{Button, Key};
use graphics;
use graphics::RelativeTransform;

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
            cells: vec![CellState::Dirt; (width * height) as usize],
            width: width,
            height: height
        }
    }
}

pub struct GameScene {
    quit: bool,
    world: World,
    camera_pos: (f64, f64)
}

impl GameScene {
    pub fn new() -> BoxedScene {
        Box::new(GameScene { 
            quit: false,
            world: World::new(10, 10),
            camera_pos: (0.0, 0.0)
        })
    }
}

impl Scene for GameScene {
    fn render(&self, gl: &mut Gl, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BROWN: [f32; 4] = [0.2, 0.2, 0.0, 1.0];
        const GREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        const CELL_SIZE: u32 = 32;

        let (camera_x, camera_y) = self.camera_pos;

        let context = &graphics::Context::abs(args.width as f64, args.height as f64)
            .trans(((args.width / 2) - (CELL_SIZE / 2)) as f64, ((args.height / 2) - (CELL_SIZE / 2)) as f64) //Camera is in the centre of the screen
            .trans(-camera_x, -camera_y);

        gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
            graphics::clear(BLACK, gl);
            for (i, cell) in self.world.cells.iter().enumerate() {
                let x = (i as u32 % self.world.width) * CELL_SIZE;
                let y = (i as u32 / self.world.height) * CELL_SIZE;

                match *cell {
                    CellState::Dirt => {
                        graphics::rectangle(BROWN, graphics::rectangle::square(x as f64, y as f64, CELL_SIZE as f64), context.transform, gl);
                    },
                    CellState::Stone => {
                        graphics::rectangle(GREY, graphics::rectangle::square(x as f64, y as f64, CELL_SIZE as f64), context.transform, gl);
                    },
                    _ => ()
                }
            }
        });

    }

    fn think(&mut self) -> Option<SceneCommand> {

        if self.quit {
            Some(SceneCommand::PopScene)
        } else {
            None
        }
    }

    fn input(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(Key::Up) => {
                let (x, y) = self.camera_pos;
                self.camera_pos = (x, y - 1.0);
            },
            &Button::Keyboard(Key::Down) => {
                let (x, y) = self.camera_pos;
                self.camera_pos = (x, y + 1.0);
            },
            &Button::Keyboard(Key::Left) => {
                let (x, y) = self.camera_pos;
                self.camera_pos = (x - 1.0, y);
            },
            &Button::Keyboard(Key::Right) => {
                let (x, y) = self.camera_pos;
                self.camera_pos = (x + 1.0, y);
            },
            _ => ()
        }
        //self.quit = true;
    }
}
