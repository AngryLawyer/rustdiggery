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

pub trait EntityType {
    fn input(&mut self, state: &mut EntityState, key: Movement, adjacents: &Adjacents);
    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Option<RcEntity>));
}

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

pub struct EntityState {
    pub x: u32,
    pub y: u32,
    pub pos_fraction: f32,
    pub movement: Movement,
    pub cell_move_state: CellMoveState,
}

pub struct Entity {
    pub state: EntityState,
    entity_type: Box<EntityType>
}

impl Entity {

    pub fn new(x: u32, y: u32, entity_type: Box<EntityType>) -> RcEntity {
        Rc::new(RefCell::new(Entity {
            state: EntityState {
                x: x,
                y: y,
                pos_fraction: 0.0,
                movement: Movement::NEUTRAL,
                cell_move_state: CellMoveState::NEUTRAL,
            },
            entity_type: entity_type
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
        match self.state.cell_move_state {
            CellMoveState::NEUTRAL => (),
            CellMoveState::EXITING => {
                if self.state.pos_fraction >= 0.5 {
                    match self.state.movement {
                        Movement::LEFT => { self.state.x -= 1},
                        Movement::RIGHT => { self.state.x += 1},
                        Movement::UP => { self.state.y -= 1},
                        Movement::DOWN => { self.state.y += 1},
                        _ => {}
                    };
                    self.state.pos_fraction = -0.4;
                    self.state.cell_move_state = CellMoveState::ENTERING;
                } else {
                    self.state.pos_fraction += 0.1;
                }
            },
            CellMoveState::ENTERING => {
                if self.state.pos_fraction >= 0.0 {
                    self.state.pos_fraction = 0.0;
                    self.state.cell_move_state = CellMoveState::NEUTRAL;
                    self.state.movement = Movement::NEUTRAL;
                } else {
                    self.state.pos_fraction += 0.1;
                }
            }
        }

    }

    pub fn collisions(&self, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Option<RcEntity>)) {
        self.entity_type.collisions(&self.state, event_bus, cell_state)
    }

    pub fn input(&mut self, key: Movement, adjacents: &Adjacents) {
        self.entity_type.input(&mut self.state, key, adjacents);
    }

    pub fn get_abs_position(&self) -> (i32, i32) {
        let (x_offset, y_offset) = match self.state.movement {
            Movement::NEUTRAL => (0.0, 0.0),
            Movement::LEFT => (-self.state.pos_fraction, 0.0),
            Movement::RIGHT => (self.state.pos_fraction, 0.0),
            Movement::UP => (0.0, -self.state.pos_fraction),
            Movement::DOWN => (0.0, self.state.pos_fraction),
        };
        (
            ((self.state.x * CELL_SIZE) as i32 + (CELL_SIZE as f32 * x_offset) as i32) as i32,
            ((self.state.y * CELL_SIZE) as i32 + (CELL_SIZE as f32 * y_offset) as i32) as i32,
        )
    }
}

pub type RcEntity = Rc<RefCell<Entity>>;
