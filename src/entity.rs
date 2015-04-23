use std::collections::HashSet;
use input::Key;
use std::rc::Rc;
use std::cell::RefCell;
use opengl_graphics::GlGraphics;
use graphics;

pub enum XMovement {
    NEUTRAL,
    LEFT,
    RIGHT
}

pub enum YMovement {
    NEUTRAL,
    UP,
    DOWN
}

pub struct Entity {
    x: u32,
    y: u32,
    x_movement: XMovement,
    y_movement: YMovement

}

impl Entity {

    pub fn new(x: u32, y: u32) -> RcEntity {
        Rc::new(RefCell::new(Entity {
            x: x,
            y: y,
            x_movement: XMovement::NEUTRAL,
            y_movement: YMovement::NEUTRAL
        }))
    }

    pub fn render(&self, context: &graphics::Context, gl: &mut GlGraphics, tick: u64) {
        let real_tick = tick / 1000 % 10;
        let fraction = real_tick as f64 / 10.0;

        let predicted_x = (self.x * 32) as f64 + match self.x_movement {
            XMovement::LEFT => { -32.0 * fraction } ,
            XMovement::RIGHT => { 32.0 * fraction },
            _ => 0.0
        };
        let predicted_y = (self.y * 32) as f64 + match self.y_movement {
            YMovement::UP => { -32.0 * fraction } ,
            YMovement::DOWN => { 32.0 * fraction },
            _ => 0.0
        };
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        graphics::rectangle(RED, graphics::rectangle::square(predicted_x, predicted_y, 32.0), context.transform, gl);
    }

    pub fn think(&mut self, tick: u64) {
        match self.x_movement {
            XMovement::LEFT => { self.x -= 1},
            XMovement::RIGHT => { self.x += 1},
            _ => ()
        };
        self.x_movement = XMovement::NEUTRAL;

        match self.y_movement {
            YMovement::UP => { self.y -= 1},
            YMovement::DOWN => { self.y += 1},
            _ => ()
        }
        self.y_movement = YMovement::NEUTRAL;
    }

    pub fn input(&mut self, key: Key) {
        match key {
            Key::Up => {
                self.y_movement = YMovement::UP;
            },
            Key::Down => {
                self.y_movement = YMovement::DOWN;
            },
            Key::Left => {
                self.x_movement = XMovement::LEFT;
            },
            Key::Right => {
                self.x_movement = XMovement::RIGHT;
            },
            _ => ()
        }
    }
}

pub type RcEntity = Rc<RefCell<Entity>>;
