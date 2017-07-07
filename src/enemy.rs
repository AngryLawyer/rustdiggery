use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_data::GameData;
use game_scene::GameEvent;
use map::{Adjacents, Adjacent, CellState};
use sdl2_engine_helpers::event_bus::EventBus;


#[derive(Clone, Copy)]
pub enum TurnDir {
    CLOCKWISE,
    ANTICLOCKWISE
}

pub struct Enemy {
    turn_dir: TurnDir,
    move_dir: Movement
}

impl Enemy {
    pub fn new(turn_dir: TurnDir) -> Box<EntityType> {
        Box::new(Enemy {
            turn_dir: turn_dir,
            move_dir: Movement::RIGHT
        })
    }

    fn considered_passable(&self, adjacent: &Adjacent) -> bool {
        match adjacent {
            &Some((CellState::Empty, ref items)) => {
                if items.len() == 0 || items.first().unwrap().borrow().is_player() {
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }

    fn set_movement(&mut self, state: &mut EntityState, movement: Movement) {
        self.move_dir = movement;
        state.set_movement(movement);
    }
}

impl EntityType for Enemy {
    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
        match cell_state {
            (_, ref items) if items.len() > 0 => {
                let item = items.first().unwrap();
                if item.borrow().is_player() {
                    event_bus.enqueue(GameEvent::Crushed(item.clone()));
                }
            },
            _ => ()
        }
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, engine_data: &GameData, tick: u64) {
        if state.cell_move_state != CellMoveState::NEUTRAL {
            return;
        }
        let grid = (
            self.considered_passable(&adjacents.left),
            self.considered_passable(&adjacents.top),
            self.considered_passable(&adjacents.right),
            self.considered_passable(&adjacents.bottom)
        );
        match (self.turn_dir, self.move_dir, grid) {
            (TurnDir::CLOCKWISE, Movement::LEFT, (_, true, _, _)) => {
                self.set_movement(state, Movement::UP);
            },
            (TurnDir::ANTICLOCKWISE, Movement::LEFT, (_, _, _, true)) => {
                self.set_movement(state, Movement::DOWN);
            },
            (_, Movement::LEFT, (true, _, _, _)) => {
                self.set_movement(state, Movement::LEFT);
            },
            (TurnDir::CLOCKWISE, Movement::LEFT, (_, _, _, true)) => {
                self.set_movement(state, Movement::DOWN);
            },
            (TurnDir::ANTICLOCKWISE, Movement::LEFT, (_, true, _, _)) => {
                self.set_movement(state, Movement::UP);
            },
            (_, Movement::LEFT, (_, _, true, _)) => {
                self.set_movement(state, Movement::RIGHT);
            },

            (TurnDir::CLOCKWISE, Movement::UP, (_, _, true, _)) => {
                self.set_movement(state, Movement::RIGHT);
            },
            (TurnDir::ANTICLOCKWISE, Movement::UP, (true, _, _, _)) => {
                self.set_movement(state, Movement::LEFT);
            },
            (_, Movement::UP, (_, true, _, _)) => {
                self.set_movement(state, Movement::UP);
            },
            (TurnDir::CLOCKWISE, Movement::UP, (true, _, _, _)) => {
                self.set_movement(state, Movement::LEFT);
            },
            (TurnDir::ANTICLOCKWISE, Movement::UP, (_, _, true, _)) => {
                self.set_movement(state, Movement::RIGHT);
            },
            (_, Movement::UP, (_, _, _, true)) => {
                self.set_movement(state, Movement::DOWN);
            },

            (TurnDir::CLOCKWISE, Movement::RIGHT, (_, _, _, true)) => {
                self.set_movement(state, Movement::DOWN);
            },
            (TurnDir::ANTICLOCKWISE, Movement::RIGHT, (_, true, _, _)) => {
                self.set_movement(state, Movement::UP);
            },
            (_, Movement::RIGHT, (_, _, true, _)) => {
                self.set_movement(state, Movement::RIGHT);
            },
            (TurnDir::CLOCKWISE, Movement::RIGHT, (_, true, _, _)) => {
                self.set_movement(state, Movement::UP);
            },
            (TurnDir::ANTICLOCKWISE, Movement::RIGHT, (_, _, _, true)) => {
                self.set_movement(state, Movement::DOWN);
            },
            (_, Movement::RIGHT, (true, _, _, _)) => {
                self.set_movement(state, Movement::LEFT);
            },

            (TurnDir::CLOCKWISE, Movement::DOWN, (true, _, _, _)) => {
                self.set_movement(state, Movement::LEFT);
            },
            (TurnDir::ANTICLOCKWISE, Movement::DOWN, (_, _, true, _)) => {
                self.set_movement(state, Movement::RIGHT);
            },
            (_, Movement::DOWN, (_, _, _, true)) => {
                self.set_movement(state, Movement::DOWN);
            },
            (TurnDir::CLOCKWISE, Movement::DOWN, (_, _, true, _)) => {
                self.set_movement(state, Movement::RIGHT);
            },
            (TurnDir::ANTICLOCKWISE, Movement::DOWN, (true, _, _, _)) => {
                self.set_movement(state, Movement::LEFT);
            },
            (_, Movement::DOWN, (_, true, _, _)) => {
                self.set_movement(state, Movement::UP);
            },
            _ => ()
        }
    }
}
