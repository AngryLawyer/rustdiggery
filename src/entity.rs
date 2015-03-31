use std::collections::HashSet;
use input::Key;
use std::rc::Rc;
use std::cell::RefCell;
use opengl_graphics::Gl;
use graphics;

pub struct Entity {
    x: u32,
    y: u32
}

impl Entity {

    pub fn new(x: u32, y: u32) -> RcEntity {
        Rc::new(RefCell::new(Entity {
            x: x,
            y: y
        }))
    }

    pub fn render(&self, context: &graphics::Context, gl: &mut Gl) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        graphics::rectangle(RED, graphics::rectangle::square((self.x * 32) as f64, (self.y * 32) as f64, 32.0), context.transform, gl);
    }

    pub fn input(&mut self, keys: &HashSet<Key>) {
        for &key in keys.iter() {
            match key {
                Key::Up => {
                    self.y -= 1;
                },
                Key::Down => {
                    self.y += 1;
                },
                Key::Left => {
                    self.x -= 1;
                },
                Key::Right => {
                    self.x += 1;
                },
                _ => ()
            }
        };
    }
}

pub type RcEntity = Rc<RefCell<Entity>>;
