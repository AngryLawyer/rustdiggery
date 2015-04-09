use std::collections::HashSet;
use scene::{Scene, BoxedScene, SceneCommand};
use opengl_graphics::Gl;
use event::{RenderArgs, UpdateArgs};
use input::{Button, Key};
use graphics;
use graphics::Transformed;
use entity::{Entity, RcEntity};
use std::cell::RefCell;

#[derive(Clone)]
enum CellState {
    Empty,
    Dirt,
    Stone,
    Wall
}

struct World {
    cells: Vec<CellState>,
    entities: Vec<RcEntity>,
    player: RcEntity,
    width: u32,
    height: u32
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        let mut entities = vec![];
        let mut cells = vec![CellState::Dirt; (width * height) as usize];
        let player = Entity::new(1,1);
        let borrow = player.clone();
        entities.push(player);
        cells[1] = CellState::Empty;
        cells[width as usize] = CellState::Empty;
        cells[width as usize + 1] = CellState::Empty;
        cells[width as usize + 2] = CellState::Empty;
        cells[(width as usize * 2) + 1] = CellState::Empty;

        World {
            cells: cells,
            entities: entities,
            width: width,
            height: height,
            player: borrow
        }
    }

    pub fn render(&self, context: &graphics::Context, gl: &mut Gl, tick: u64) {
        for entity in self.entities.iter() {
            entity.borrow().render(context, gl, tick);
        }
    }
}

pub struct GameScene {
    quit: bool,
    world: World,
    keys: HashSet<Key>,
    last_keypress: Option<Key>,
    tick: u64,
    camera_pos: (f64, f64)
}

impl GameScene {
    pub fn new() -> BoxedScene {
        Box::new(GameScene { 
            quit: false,
            world: World::new(100, 100),
            keys: HashSet::new(),
            last_keypress: None,
            tick: 0,
            camera_pos: (0.0, 0.0)
        })
    }

    fn get_last_keypress(&self) -> Option<Key> {
        for &key in self.keys.iter() {
            match key {
                Key::Up | Key::Down | Key::Left | Key::Right => {
                    return Some(key);
                },
                _ => ()
            }
        };
        None
    }

}

impl Scene for GameScene {
    fn render(&self, gl: &mut Gl, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BROWN: [f32; 4] = [0.2, 0.2, 0.0, 1.0];
        const GREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        const WHITE: [f32; 4] = [1.0; 4];
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
                    CellState::Wall => {
                        graphics::rectangle(WHITE, graphics::rectangle::square(x as f64, y as f64, CELL_SIZE as f64), context.transform, gl);
                    }
                    _ => ()
                }
            };
            self.world.render(context, gl, self.tick);
        });

    }
    fn think(&mut self, args: &UpdateArgs) -> Option<SceneCommand> {
        self.tick += (args.dt * 100000.0) as u64;

        match self.get_last_keypress() {
            Some(key) => self.last_keypress = Some(key),
            _ => ()
        };

        if self.tick / 1000 % 10 == 0 {
            //FIXME: Make this use weak references once we have them
            let entity = self.world.player.clone();
            //Move existing
            entity.borrow_mut().think(self.tick);
            //Handle player input
            //FIXME: Make player input less painful
            entity.borrow_mut().input(&self.last_keypress);
            self.last_keypress = None;
        }


        if self.quit {
            Some(SceneCommand::PopScene)
        } else {
            None
        }
    }

    fn press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => {
                self.keys.insert(key)
            },
            _ => false
        };
        //self.quit = true;
    }

    fn release(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => {
                self.keys.remove(&key)
            },
            _ => false
        };
    }
}
