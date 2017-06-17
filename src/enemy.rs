use map::{CELL_SIZE, Adjacents};
use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;

pub enum TurnDir {
    CLOCKWISE,
    ANTICLOCKWISE
}

pub struct Enemy {
    turn_dir: TurnDir
}

impl Enemy {
    pub fn new(turn_dir: TurnDir) -> Box<EntityType> {
        Box::new(Enemy {
            turn_dir: turn_dir
        })
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
        // TODO: Movement
    }
}
