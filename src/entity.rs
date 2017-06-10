use map::{CELL_SIZE, Adjacents};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use transform::TransformContext;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;
use game_scene::GameEvent;

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
    pub pos_fraction_x: f32,
    pub pos_fraction_y: f32,
    movement: Movement,
}

impl Entity {

    pub fn new(x: u32, y: u32) -> RcEntity {
        Rc::new(RefCell::new(Entity {
            x: x,
            y: y,
            pos_fraction_x: 0.0,
            pos_fraction_y: 0.0,
            movement: Movement::NEUTRAL,
        }))
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, engine_data: &(), tick: u64) {
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        transform.fill_rect(
            renderer,
            Rect::new(
                ((self.x * CELL_SIZE) as i32 + (CELL_SIZE as f32 * self.pos_fraction_x) as i32) as i32,
                ((self.y * CELL_SIZE) as i32 + (CELL_SIZE as f32 * self.pos_fraction_y) as i32) as i32,
                CELL_SIZE,
                CELL_SIZE
            )
        );
    }

    /*pub fn render(&self, context: &graphics::Context, gl: &mut GlGraphics, tick: u64) {
        let real_tick = tick / 1000 % 10;
        let fraction = real_tick as f64 / 10.0;

        let predicted_x = (self.x * 32) as f64 + match self.movement {
            Movement::LEFT => { -32.0 * fraction },
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
    }*/

    pub fn think(&mut self, tick: u64) {
        if self.pos_fraction_x >= 0.9 || self.pos_fraction_x <= -0.9  ||
            self.pos_fraction_y >= 0.9 || self.pos_fraction_y <= -0.9 {
            match self.movement {
                Movement::LEFT => { self.x -= 1},
                Movement::RIGHT => { self.x += 1},
                Movement::UP => { self.y -= 1},
                Movement::DOWN => { self.y += 1},
                _ => {}
            };
            self.pos_fraction_x = 0.0;
            self.pos_fraction_y = 0.0;
            self.movement = Movement::NEUTRAL;
        } else {
            match self.movement {
                Movement::LEFT => { self.pos_fraction_x -= 0.1},
                Movement::RIGHT => { self.pos_fraction_x += 0.1},
                Movement::UP => { self.pos_fraction_y -= 0.1},
                Movement::DOWN => { self.pos_fraction_y += 0.1},
                _ => {}
            };
        }
    }

    pub fn collisions(&self, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Option<RcEntity>)) {
        match cell_state {
            (CellState::Dirt, _) => {
                event_bus.enqueue(GameEvent::DigEvent(self.x, self.y));
            },
            _ => ()
        }
    }

    pub fn input(&mut self, key: Movement, adjacents: &Adjacents) {
        if self.pos_fraction_x != 0.0 || self.pos_fraction_y != 0.0 {
            return;
        }

        match key {
            Movement::UP => {
                match adjacents.top {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::UP;
                    },
                    _ => ()
                }
            },
            Movement::DOWN => {
                match adjacents.bottom {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::DOWN;
                    },
                    _ => ()
                }
            },
            Movement::LEFT => {
                match adjacents.left {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::LEFT;
                    },
                    _ => ()
                }
            },
            Movement::RIGHT => {
                match adjacents.right {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::RIGHT;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

pub type RcEntity = Rc<RefCell<Entity>>;
