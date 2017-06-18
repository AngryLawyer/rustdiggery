use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::{CELL_SIZE, Adjacents, CellState};
use sdl2_engine_helpers::event_bus::EventBus;


#[derive(Clone, Copy)]
pub enum TurnDir {
    CLOCKWISE,
    ANTICLOCKWISE
}

pub struct Enemy {
    turn_dir: TurnDir,
    current_dir: Movement
}

impl Enemy {
    pub fn new(turn_dir: TurnDir) -> Box<EntityType> {
        Box::new(Enemy {
            turn_dir: turn_dir,
            current_dir: Movement::RIGHT
        })
    }

    fn turn(&mut self) {
        match (self.turn_dir, self.current_dir) {
            (TurnDir::CLOCKWISE, Movement::RIGHT) => {
                self.current_dir = Movement::DOWN;
            },
            (TurnDir::CLOCKWISE, Movement::DOWN) => {
                self.current_dir = Movement::LEFT;
            },
            (TurnDir::CLOCKWISE, Movement::LEFT) => {
                self.current_dir = Movement::UP;
            },
            (TurnDir::CLOCKWISE, Movement::UP) => {
                self.current_dir = Movement::RIGHT;
            },
            (TurnDir::ANTICLOCKWISE, Movement::RIGHT) => {
                self.current_dir = Movement::UP;
            },
            (TurnDir::ANTICLOCKWISE, Movement::DOWN) => {
                self.current_dir = Movement::RIGHT;
            },
            (TurnDir::ANTICLOCKWISE, Movement::LEFT) => {
                self.current_dir = Movement::DOWN;
            },
            (TurnDir::ANTICLOCKWISE, Movement::UP) => {
                self.current_dir = Movement::LEFT;
            },
            _ => ()
        }
    }

    fn check_move(&mut self, state: &mut EntityState, adjacents: &Adjacents) -> bool {
        let adjacent = match self.current_dir {
            Movement::NEUTRAL | Movement::RIGHT => &adjacents.right,
            Movement::UP => &adjacents.top,
            Movement::LEFT => &adjacents.left,
            Movement::DOWN => &adjacents.bottom,
        };
        match adjacent {
            &Some((CellState::Empty, ref items)) => {
                if items.len() == 0 || items.first().unwrap().borrow().is_player() {
                    state.movement = self.current_dir;
                    state.cell_move_state = CellMoveState::EXITING;
                    true
                } else {
                    false
                }
            },
            _ => false
        }
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

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
        if state.cell_move_state != CellMoveState::NEUTRAL {
            return;
        }
        let mut count = 0;
        while count < 4 {
            match self.current_dir {
                Movement::NEUTRAL => {
                    self.current_dir = Movement::RIGHT;
                },
                _ => {
                    if self.check_move(state, adjacents) {
                        return;
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }
}
