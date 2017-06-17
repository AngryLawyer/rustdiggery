use map::{CELL_SIZE, Adjacents};
use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;

pub struct Player {
    pushing: Movement,
}

impl Player {
    pub fn new() -> Box<EntityType> {
        Box::new(Player {
            pushing: Movement::NEUTRAL,
        })
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
                    Some((ref tile, ref items)) if tile.is_passable() && items.len() == 0 => {
                        state.movement = Movement::UP;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    Some((CellState::Empty, ref items)) if items.len() == 1 => {
                        let item = items.first().unwrap().borrow();
                        if item.is_enterable() {
                            state.movement = Movement::UP;
                            state.cell_move_state = CellMoveState::EXITING;
                        }
                    },
                    _ => ()
                }
            },
            Movement::DOWN => {
                match adjacents.bottom {
                    Some((ref tile, ref items)) if tile.is_passable() && items.len() == 0 => {
                        state.movement = Movement::DOWN;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    Some((CellState::Empty, ref items)) if items.len() == 1 => {
                        let item = items.first().unwrap().borrow();
                        if item.is_enterable() {
                            state.movement = Movement::DOWN;
                            state.cell_move_state = CellMoveState::EXITING;
                        }
                    },
                    _ => ()
                }
            },
            Movement::LEFT => {
                match adjacents.left {
                    Some((ref tile, ref items)) if tile.is_passable() && items.len() == 0 => {
                        state.movement = Movement::LEFT;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    Some((CellState::Empty, ref items)) if items.len() == 1 => {
                        let item = items.first().unwrap().borrow();
                        if item.is_enterable() {
                            state.movement = Movement::LEFT;
                            state.cell_move_state = CellMoveState::EXITING;
                        } else {
                            self.pushing = Movement::LEFT;
                        }
                    },
                    _ => ()
                }
            },
            Movement::RIGHT => {
                match adjacents.right {
                    Some((ref tile, ref items)) if tile.is_passable() && items.len() == 0 => {
                        state.movement = Movement::RIGHT;
                        state.cell_move_state = CellMoveState::EXITING;
                    },
                    Some((CellState::Empty, ref items)) if items.len() == 1 => {
                        let item = items.first().unwrap().borrow();
                        if item.is_enterable() {
                            state.movement = Movement::RIGHT;
                            state.cell_move_state = CellMoveState::EXITING;
                        } else {
                            self.pushing = Movement::RIGHT;
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
        match cell_state {
            (CellState::Dirt, _) => {
                event_bus.enqueue(GameEvent::Dig(state.x, state.y));
            },
            (_, ref items) if items.len() > 0 => {
                let item = items.first().unwrap();
                if item.borrow().is_collectible() {
                    event_bus.enqueue(GameEvent::Collect(item.clone()));
                }
            },
            _ => ()
        }
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
        match (&self.pushing, &adjacents.left, &adjacents.right) {
            (&Movement::LEFT, &Some((_, ref items)), _) if items.len() > 0 => {
                let item = items.first().unwrap();
                if item.borrow().is_hard() {
                    event_bus.enqueue(GameEvent::Push(Movement::LEFT, item.clone()));
                }
            },
            (&Movement::RIGHT, _, &Some((_, ref items))) if items.len() > 0 => {
                let item = items.first().unwrap();
                if item.borrow().is_hard() {
                    event_bus.enqueue(GameEvent::Push(Movement::RIGHT, item.clone()));
                }
            },
            _ => ()
        };
        self.pushing = Movement::NEUTRAL;
    }

    fn is_hard(&self) -> bool {
        false
    }

    fn is_enterable(&self) -> bool {
        false
    }

    fn push(&mut self, direction: Movement, tick: u64) {
    }

    fn is_player(&self) -> bool {
        true
    }
}
