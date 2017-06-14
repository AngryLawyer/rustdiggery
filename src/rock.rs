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

        match (
            &adjacents.left,
            &adjacents.right,
            &adjacents.bottom_left,
            &adjacents.bottom,
            &adjacents.bottom_right
        ) {
            (_, _, _, &Some((CellState::Empty, None)), _) => {
                state.movement = Movement::DOWN;
                state.cell_move_state = CellMoveState::EXITING;
            },
            (_, &Some((CellState::Empty, None)), _, &Some((CellState::Empty, Some(ref underneath))), &Some((CellState::Empty, None))) => {
                if underneath.borrow().is_hard() {
                    state.movement = Movement::RIGHT;
                    state.cell_move_state = CellMoveState::EXITING;
                } else {
                    state.movement = Movement::NEUTRAL;
                }
            },
            (&Some((CellState::Empty, None)), _, &Some((CellState::Empty, None)), &Some((CellState::Empty, Some(ref underneath))), _) => {
                if underneath.borrow().is_hard() {
                    state.movement = Movement::LEFT;
                    state.cell_move_state = CellMoveState::EXITING;
                } else {
                    state.movement = Movement::NEUTRAL;
                }
            },
            _ => {
                state.movement = Movement::NEUTRAL;
            }
        };
    }

    fn is_hard(&self) -> bool {
        true
    }
}
