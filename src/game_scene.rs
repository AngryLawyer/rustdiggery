use scene::{Scene, BoxedScene, SceneCommand};
use opengl_graphics::GlGraphics;
use event::{RenderArgs, UpdateArgs};
use input::{Button, Key};
use graphics;
use graphics::Transformed;
use entity::{Entity, RcEntity};
use keyhandler::KeyHandler;
use std::cell::RefCell;

#[derive(Clone)]
pub enum CellState {
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

pub struct Adjacents { 
    pub top: Option<(CellState, Option<RcEntity>)>,
    pub left: Option<(CellState, Option<RcEntity>)>,
    pub bottom: Option<(CellState, Option<RcEntity>)>,
    pub right: Option<(CellState, Option<RcEntity>)>
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

    pub fn render(&self, context: &graphics::Context, gl: &mut GlGraphics, tick: u64) {
        for entity in self.entities.iter() {
            entity.borrow().render(context, gl, tick);
        }
    }

    pub fn at_pos(&self, x: u32, y: u32) -> (CellState, Option<RcEntity>) {
        let index = x + (y * self.width);
        return (self.cells[index as usize].clone(), None);
    }
    
    pub fn adjacents(&self, x: u32, y: u32) -> Adjacents {
        let top = if y > 0 {
            Some(self.at_pos(x, y - 1))
        } else {
            None
        };

        let left = if x > 0 {
            Some(self.at_pos(x - 1, y))
        } else {
            None
        };

        let bottom = if y < self.height - 1 {
            Some(self.at_pos(x, y + 1))
        } else {
            None
        };

        let right = if x < self.width - 1 {
            Some(self.at_pos(x + 1, y))
        } else {
            None
        };

        Adjacents{top: top, left: left, bottom: bottom, right: right}
    }

    pub fn set_pos(&mut self, x: u32, y: u32, state: CellState) {
        let index = x + (y * self.width);
        self.cells[index as usize] = state;
    }
}

pub struct GameScene {
    quit: bool,
    world: World,
    keyhandler: KeyHandler,
    tick: u64,
    camera_pos: (f64, f64)
}

impl GameScene {
    pub fn new() -> BoxedScene {
        Box::new(GameScene { 
            quit: false,
            world: World::new(10, 10),
            keyhandler: KeyHandler::new(),
            tick: 0,
            camera_pos: (0.0, 0.0)
        })
    }
}

impl Scene for GameScene {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BROWN: [f32; 4] = [0.2, 0.2, 0.0, 1.0];
        const GREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        const WHITE: [f32; 4] = [1.0; 4];
        const CELL_SIZE: u32 = 32;

        let (camera_x, camera_y) = self.camera_pos;

        let context = &graphics::Context::abs(args.width as f64, args.height as f64)
            .trans(((args.width / 2) - (CELL_SIZE / 2)) as f64, ((args.height / 2) - (CELL_SIZE / 2)) as f64) //Camera is in the centre of the screen
            .trans(-camera_x, -camera_y);

        gl.draw(args.viewport(), |_, gl| {
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
        self.keyhandler.think(self.tick);

        if self.tick / 1000 % 10 == 0 {
            //FIXME: Make this use weak references once we have them
            let entity = self.world.player.clone();
            let mut entity = entity.borrow_mut();
            //Move existing
            let x = entity.x;
            let y = entity.y;
            entity.think(self.tick, &self.world.adjacents(x, y));

            let x = entity.x;
            let y = entity.y;
            self.world.set_pos(x, y, CellState::Empty);

            //Handle player input
            match self.keyhandler.last_key() {
                Some((key, tick)) => {
                    let difference = self.tick - tick;
                    if difference < 8000 || difference > 20000 {
                        let x = entity.x;
                        let y = entity.y;
                        entity.input(key, &self.world.adjacents(x, y));
                    }
                },
                None => ()
            }
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
                self.keyhandler.press(key)
            },
            _ => ()
        };
        //self.quit = true;
    }

    fn release(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => {
                self.keyhandler.release(key)
            },
            _ => ()
        };
    }
}
