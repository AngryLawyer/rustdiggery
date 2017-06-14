use map::{CELL_SIZE, Adjacents};
use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;

pub struct Rock {
    momentum: bool,
}

impl Rock {
    pub fn new() -> Box<EntityType> {
        Box::new(Rock {
            momentum: false
        })
    }
}

pub fn handle_collisions(state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
    match cell_state {
        (_, ref items) if items.len() > 0 => {
            for item in items {
                if !item.borrow().is_hard() {
                    event_bus.enqueue(GameEvent::Crushed(item.clone()));
                }
            }
        },
        _ => ()
    }
}

pub fn think(state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64, momentum: &mut bool) {
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
        (_, _, _, &Some((CellState::Empty, ref items)), _) if items.len() == 0 => {
            state.movement = Movement::DOWN;
            state.cell_move_state = CellMoveState::EXITING;
            *momentum = true;
        },
        (_, _, _, &Some((CellState::Empty, ref items)), _) if items.len() > 0 && *momentum => {
            if !items.first().unwrap().borrow().is_hard() {
                state.movement = Movement::DOWN;
                state.cell_move_state = CellMoveState::EXITING;
                *momentum = true;
            } else {
                state.movement = Movement::NEUTRAL;
                *momentum = false;
            }
        },
        (_, &Some((CellState::Empty, ref right_items)), _, &Some((CellState::Empty, ref underneath)), &Some((CellState::Empty, ref bottom_right_items))) if right_items.len() == 0 && bottom_right_items.len() == 0 => {
            if underneath.first().unwrap().borrow().is_hard() {
                state.movement = Movement::RIGHT;
                state.cell_move_state = CellMoveState::EXITING;
                *momentum = true;
            } else {
                state.movement = Movement::NEUTRAL;
                *momentum = false;
            }
        },
        (&Some((CellState::Empty, ref left_items)), _, &Some((CellState::Empty, ref bottom_left_items)), &Some((CellState::Empty, ref underneath)), _) if left_items.len() == 0 && bottom_left_items.len() == 0 => {
            if underneath.first().unwrap().borrow().is_hard() {
                state.movement = Movement::LEFT;
                state.cell_move_state = CellMoveState::EXITING;
                *momentum = true;
            } else {
                state.movement = Movement::NEUTRAL;
                *momentum = false;
            }
        },
        _ => {
            state.movement = Movement::NEUTRAL;
            *momentum = false;
        }
    };
}

impl EntityType for Rock {
    fn input(&mut self, state: &mut EntityState, key: Movement, adjacents: &Adjacents) {
    }

    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
        handle_collisions(state, event_bus, cell_state);
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
        think(state, event_bus, adjacents, tick, &mut self.momentum);
    }

    fn is_hard(&self) -> bool {
        true
    }
}
