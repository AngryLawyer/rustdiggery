use std::collections::HashSet;
use input::Key;
use std::rc::Rc;
use std::cell::RefCell;
use opengl_graphics::GlGraphics;
use graphics;
use game_scene::Adjacents;

pub enum Movement {
    NEUTRAL,
    LEFT,
    RIGHT,
    UP,
    DOWN
}

pub struct Entity {
    pub x: u32,
    pub y: u32,
    movement: Movement,
}

impl Entity {

    pub fn new(x: u32, y: u32) -> RcEntity {
        Rc::new(RefCell::new(Entity {
            x: x,
            y: y,
            movement: Movement::NEUTRAL,
        }))
    }

    pub fn render(&self, context: &graphics::Context, gl: &mut GlGraphics, tick: u64) {
        let real_tick = tick / 1000 % 10;
        let fraction = real_tick as f64 / 10.0;

        let predicted_x = (self.x * 32) as f64 + match self.movement {
            Movement::LEFT => { -32.0 * fraction } ,
            Movement::RIGHT => { 32.0 * fraction },
            _ => 0.0
        };
        let predicted_y = (self.y * 32) as f64 + match self.movement {
            Movement::UP => { -32.0 * fraction } ,
            Movement::DOWN => { 32.0 * fraction },
            _ => 0.0
        };
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        graphics::rectangle(RED, graphics::rectangle::square(predicted_x, predicted_y, 32.0), context.transform, gl);
    }

    pub fn think(&mut self, tick: u64) {
        match self.movement {
            Movement::LEFT => { self.x -= 1},
            Movement::RIGHT => { self.x += 1},
            Movement::UP => { self.y -= 1},
            Movement::DOWN => { self.y += 1},
        };
        self.movement = Movement::NEUTRAL;
    }

    pub fn input(&mut self, key: Key, adjacents: &Adjacents) {
        match key {
            Key::Up => {
                if adjacents.top.is_some() {
                    self.movement = Movement::UP;
                }
            },
            Key::Down => {
                if adjacents.bottom.is_some() {
                    self.movement = Movement::DOWN;
                }
            },
            Key::Left => {
                if adjacents.left.is_some() {
                    self.movement = Movement::LEFT;
                }
            },
            Key::Right => {
                if adjacents.right.is_some() {
                    self.movement = Movement::RIGHT;
                }
            },
            _ => ()
        }
    }
}

pub type RcEntity = Rc<RefCell<Entity>>;
