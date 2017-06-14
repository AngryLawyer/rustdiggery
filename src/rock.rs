use map::{CELL_SIZE, Adjacents};
use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;

pub struct Rock;

impl Rock {
    pub fn new() -> Box<EntityType> {
        Box::new(Rock)
    }
}

impl EntityType for Rock {
    fn input(&mut self, state: &mut EntityState, key: Movement, adjacents: &Adjacents) {
    }

    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Option<RcEntity>)) {
        match cell_state {
            _ => ()
        }
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
        if state.pos_fraction != 0.0 {
            return;
        }

        match adjacents.bottom {
            Some((CellState::Empty, _)) => {
                state.movement = Movement::DOWN;
                state.cell_move_state = CellMoveState::EXITING;
            },
            _ => {
                state.movement = Movement::NEUTRAL;
            }
        }
    }
}
