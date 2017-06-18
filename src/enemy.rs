use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::{CELL_SIZE, Adjacents, Adjacent, CellState};
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
        let grid = (
            self.considered_passable(&adjacents.left),
            self.considered_passable(&adjacents.top),
            self.considered_passable(&adjacents.right),
            self.considered_passable(&adjacents.bottom)
        );
        match (self.turn_dir, self.move_dir, grid) {
            // TODO: Maze algorythm
            _ => ()
        }
    }
}
