use map::{CELL_SIZE, Adjacents};
use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;

pub struct Player;

impl Player {
    pub fn new() -> Box<EntityType> {
        Box::new(Player)
    }
}

impl EntityType for Player {
    fn input(&mut self, state: &mut EntityState, key: Movement, adjacents: &Adjacents) {
        if state.pos_fraction != 0.0 {
            return;
        }

        match key {
            Movement::UP => {
                match adjacents.top {
                    Some((ref tile, None)) if tile.is_passable() => {
                        state.movement = Movement::UP;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            Movement::DOWN => {
                match adjacents.bottom {
                    Some((ref tile, None)) if tile.is_passable() => {
                        state.movement = Movement::DOWN;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            Movement::LEFT => {
                match adjacents.left {
                    Some((ref tile, None)) if tile.is_passable() => {
                        state.movement = Movement::LEFT;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            Movement::RIGHT => {
                match adjacents.right {
                    Some((ref tile, None)) if tile.is_passable() => {
                        state.movement = Movement::RIGHT;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Option<RcEntity>)) {
        match cell_state {
            (CellState::Dirt, _) => {
                event_bus.enqueue(GameEvent::Dig(state.x, state.y));
            },
            _ => ()
        }
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
    }

    fn is_hard(&self) -> bool {
        false
    }
}
