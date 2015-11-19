use scene::{Scene, BoxedScene, SceneCommand};
use piston_window::{PistonWindow, UpdateArgs, UpdateEvent, Context, G2d, Transformed};
use piston_window;
//use event::{RenderArgs, UpdateArgs};
//use input::{Button, Key};
//use graphics::Transformed;
//use entity::{Entity, RcEntity};
//use keyhandler::KeyHandler;
use std::cell::RefCell;

#[derive(Clone)]
pub enum CellState {
    Empty,
    Dirt,
    Stone,
    Wall
}

impl CellState {
    pub fn is_passable(&self) -> bool {
        match *self {
            CellState::Empty | CellState::Dirt => true,
            _ => false
        }
    }
}

struct World {
    cells: Vec<CellState>,
    //entities: Vec<RcEntity>,
    //player: RcEntity,
    width: u32,
    height: u32
}

/*pub struct Adjacents {
    pub top: Option<(CellState, Option<RcEntity>)>,
    pub left: Option<(CellState, Option<RcEntity>)>,
    pub bottom: Option<(CellState, Option<RcEntity>)>,
    pub right: Option<(CellState, Option<RcEntity>)>
}*/

impl World {
    pub fn new(width: u32, height: u32) -> World {
        //let mut entities = vec![];
        let mut cells = vec![CellState::Dirt; (width * height) as usize];
        //let player = Entity::new(1,1);
        //let borrow = player.clone();
        //entities.push(player);
        cells[1] = CellState::Empty;
        cells[width as usize] = CellState::Empty;
        cells[width as usize + 1] = CellState::Empty;
        cells[width as usize + 2] = CellState::Empty;
        cells[(width as usize * 2) + 1] = CellState::Stone;

        World {
            cells: cells,
            //entities: entities,
            width: width,
            height: height,
            //player: borrow
        }
    }

    /*pub fn render(&self, context: &graphics::Context, gl: &mut GlGraphics, tick: u64) {
        for entity in self.entities.iter() {
            entity.borrow().render(context, gl, tick);
        }
    }*/

    /*pub fn at_pos(&self, x: u32, y: u32) -> (CellState, Option<RcEntity>) {
        let index = x + (y * self.width);
        return (self.cells[index as usize].clone(), None);
    }*/

    /*pub fn adjacents(&self, x: u32, y: u32) -> Adjacents {
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
    }*/

    pub fn set_pos(&mut self, x: u32, y: u32, state: CellState) {
        let index = x + (y * self.width);
        self.cells[index as usize] = state;
    }
}

pub struct GameScene {
    quit: bool,
    world: World,
    //keyhandler: KeyHandler,
    tick: u64,
    next_think: u64,
    camera_pos: (f64, f64)
}

impl GameScene {
    pub fn new() -> BoxedScene {
        Box::new(GameScene {
            quit: false,
            world: World::new(10, 10),
            //keyhandler: KeyHandler::new(),
            tick: 0,
            next_think: 0,
            camera_pos: (0.0, 0.0)
        })
    }

    /*fn adjust_camera_position(&mut self) {
        let (old_x, old_y) = self.camera_pos;
        let player = self.world.player.borrow();
        self.camera_pos = ((player.x * 32)  as f64, (player.y * 32) as f64);
    }*/

    fn render(&self, context: Context, gl: &mut G2d) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const BROWN: [f32; 4] = [0.2, 0.2, 0.0, 1.0];
        const GREY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        const WHITE: [f32; 4] = [1.0; 4];
        const CELL_SIZE: u32 = 32;

        let (camera_x, camera_y) = self.camera_pos;
        piston_window::clear(BLACK, gl);
        let size = context.get_view_size();
        let width = size[0];
        let height = size[1];

        let context = context
            .trans(((width as u64 / 2) - (CELL_SIZE as u64 / 2)) as f64, ((height as u64 / 2) - (CELL_SIZE as u64 / 2)) as f64) //Camera is in the centre of the screen
            .trans(-camera_x, -camera_y);

        for (i, cell) in self.world.cells.iter().enumerate() {
            let x = (i as u32 % self.world.width) * CELL_SIZE;
            let y = (i as u32 / self.world.height) * CELL_SIZE;

            match *cell {
                CellState::Dirt => {
                    piston_window::rectangle(BROWN, piston_window::rectangle::square(x as f64, y as f64, CELL_SIZE as f64), context.transform, gl);
                },
                CellState::Stone => {
                    piston_window::rectangle(GREY, piston_window::rectangle::square(x as f64, y as f64, CELL_SIZE as f64), context.transform, gl);
                },
                CellState::Wall => {
                    piston_window::rectangle(WHITE, piston_window::rectangle::square(x as f64, y as f64, CELL_SIZE as f64), context.transform, gl);
                }
                _ => ()
            }
        };
        //self.world.render(context, gl, self.tick);

    }

    fn think(&mut self, args: &UpdateArgs) -> Option<SceneCommand> {
        self.tick += (args.dt * 100000.0) as u64;
        //self.keyhandler.think(self.tick);

        if self.tick >= self.next_think {
            self.next_think += 10000;
            {
                //FIXME: Make this use weak references once we have them
                /*let entity = self.world.player.clone();
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
                }*/
            }

            //self.adjust_camera_position();
        }

        if self.quit {
            Some(SceneCommand::PopScene)
        } else {
            None
        }
    }

    /*fn press(&mut self, button: &Button) {
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
    }*/
}

impl Scene for GameScene {
    fn handle_event(&mut self, e: &PistonWindow) -> Option<SceneCommand> {
        if let Some(u) = e.update_args() {
            self.think(&u)
        } else {
            e.draw_2d(|context, g| {
                self.render(context, g);
            });
            None
        }
    }
}
