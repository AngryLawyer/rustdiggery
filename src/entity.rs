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

pub enum CellMoveState {
    NEUTRAL,
    ENTERING,
    EXITING
}

pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub pos_fraction: f32,
    movement: Movement,
    cell_move_state: CellMoveState,
}

impl Entity {

    pub fn new(x: u32, y: u32) -> RcEntity {
        Rc::new(RefCell::new(Entity {
            x: x,
            y: y,
            pos_fraction: 0.0,
            movement: Movement::NEUTRAL,
            cell_move_state: CellMoveState::NEUTRAL,
        }))
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, engine_data: &(), tick: u64) {
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        let (x, y) = self.get_abs_position();
        transform.fill_rect(
            renderer,
            Rect::new(
                x,
                y,
                CELL_SIZE,
                CELL_SIZE
            )
        );
    }

    pub fn think(&mut self, tick: u64) {
        // TODO: This should be an event
        match self.cell_move_state {
            CellMoveState::NEUTRAL => (),
            CellMoveState::EXITING => {
                if self.pos_fraction >= 0.5 {
                    match self.movement {
                        Movement::LEFT => { self.x -= 1},
                        Movement::RIGHT => { self.x += 1},
                        Movement::UP => { self.y -= 1},
                        Movement::DOWN => { self.y += 1},
                        _ => {}
                    };
                    self.pos_fraction = -0.4;
                    self.cell_move_state = CellMoveState::ENTERING;
                } else {
                    self.pos_fraction += 0.1;
                }
            },
            CellMoveState::ENTERING => {
                if self.pos_fraction >= 0.0 {
                    self.pos_fraction = 0.0;
                    self.cell_move_state = CellMoveState::NEUTRAL;
                    self.movement = Movement::NEUTRAL;
                } else {
                    self.pos_fraction += 0.1;
                }
            }
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
        if self.pos_fraction != 0.0 {
            return;
        }

        match key {
            Movement::UP => {
                match adjacents.top {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::UP;
                        self.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            Movement::DOWN => {
                match adjacents.bottom {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::DOWN;
                        self.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            Movement::LEFT => {
                match adjacents.left {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::LEFT;
                        self.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            Movement::RIGHT => {
                match adjacents.right {
                    Some((ref tile, _)) if tile.is_passable() => {
                        self.movement = Movement::RIGHT;
                        self.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    pub fn get_abs_position(&self) -> (i32, i32) {
        let (x_offset, y_offset) = match self.movement {
            Movement::NEUTRAL => (0.0, 0.0),
            Movement::LEFT => (-self.pos_fraction, 0.0),
            Movement::RIGHT => (self.pos_fraction, 0.0),
            Movement::UP => (0.0, -self.pos_fraction),
            Movement::DOWN => (0.0, self.pos_fraction),
        };
        (
            ((self.x * CELL_SIZE) as i32 + (CELL_SIZE as f32 * x_offset) as i32) as i32,
            ((self.y * CELL_SIZE) as i32 + (CELL_SIZE as f32 * y_offset) as i32) as i32,
        )
    }
}

pub type RcEntity = Rc<RefCell<Entity>>;
