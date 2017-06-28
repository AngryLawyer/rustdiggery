use map::{CELL_SIZE, Adjacents};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;
use transform::TransformContext;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;
use game_scene::GameEvent;
use game_data::GameData;

pub trait EntityType {
    fn input(&mut self, state: &mut EntityState, key: Movement, adjacents: &Adjacents) {
    }
    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
    }
    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
    }
    fn is_hard(&self) -> bool {
        false
    }
    fn push(&mut self, direction: Movement, tick: u64) {
    }
    fn is_enterable(&self) -> bool {
        false
    }
    fn is_collectible(&self) -> bool {
        false
    }
    fn is_player(&self) -> bool {
        false
    }
    fn score(&self) -> u32 {
        0
    }
    fn open_exit(&mut self) {
    }
    fn destructable(&self) -> bool {
        true
    }
    fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, engine_data: &GameData, tick: u64) {
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        transform.fill_rect(
            renderer,
            Rect::new(
                0,
                0,
                CELL_SIZE,
                CELL_SIZE
            )
        ).expect("Failed to draw entity");
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Movement {
    NEUTRAL,
    LEFT,
    RIGHT,
    UP,
    DOWN
}

#[derive(Eq, PartialEq)]
pub enum CellMoveState {
    NEUTRAL,
    ENTERING,
    EXITING
}

pub struct EntityState {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub destroyed: bool,
    pub pos_fraction: f32,
    pub movement: Movement,
    pub cell_move_state: CellMoveState,
}

impl EntityState {
    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
        self.cell_move_state = CellMoveState::EXITING;
    }
}

pub struct Entity {
    pub state: EntityState,
    entity_type: Box<EntityType>
}

impl Entity {

    pub fn new(x: u32, y: u32, entity_type: Box<EntityType>, id: &mut u32) -> RcEntity {
        let item = Rc::new(RefCell::new(Entity {
            state: EntityState {
                x: x,
                y: y,
                pos_fraction: 0.0,
                movement: Movement::NEUTRAL,
                cell_move_state: CellMoveState::NEUTRAL,
                destroyed: false,
                id: *id
            },
            entity_type: entity_type
        }));
        *id += 1;
        item
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, engine_data: &GameData, tick: u64) {
        self.entity_type.render(renderer, transform, engine_data, tick);
    }

    pub fn think(&mut self, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
        self.entity_type.think(&mut self.state, event_bus, adjacents, tick);
    }

    pub fn process(&mut self, tick: u64) {
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

    pub fn collisions(&self, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
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

    pub fn is_hard(&self) -> bool {
        self.entity_type.is_hard()
    }

    pub fn target_square(&self) -> Option<(u32, u32)> {
        match self.state.cell_move_state {
            CellMoveState::EXITING => {
                Some(match self.state.movement {
                    Movement::NEUTRAL => (self.state.x, self.state.y),
                    Movement::LEFT => (self.state.x - 1, self.state.y),
                    Movement::RIGHT => (self.state.x + 1, self.state.y),
                    Movement::UP => (self.state.x, self.state.y - 1),
                    Movement::DOWN => (self.state.x, self.state.y + 1),
                })
            },
            _ => None
        }
    }

    pub fn deflect(&mut self) {
        match self.state.cell_move_state {
            CellMoveState::EXITING => {
                self.state.cell_move_state = CellMoveState::ENTERING;
                self.state.pos_fraction = 1.0 - self.state.pos_fraction;
            },
            _ => ()
        }
    }

    pub fn destroy(&mut self) {
        self.state.destroyed = true;
    }

    pub fn push(&mut self, direction: Movement, tick: u64) {
        self.entity_type.push(direction, tick);
    }

    pub fn is_enterable(&self) -> bool {
        self.entity_type.is_enterable()
    }

    pub fn is_collectible(&self) -> bool {
        self.entity_type.is_collectible()
    }

    pub fn is_player(&self) -> bool {
        self.entity_type.is_player()
    }

    pub fn score(&self) -> u32 {
        self.entity_type.score()
    }

    pub fn open_exit(&mut self) {
        self.entity_type.open_exit()
    }

    pub fn destructable(&self) -> bool {
        self.entity_type.destructable()
    }
}

pub type RcEntity = Rc<RefCell<Entity>>;
