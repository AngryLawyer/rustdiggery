use map::{CELL_SIZE, Adjacents};
use entity::{EntityType, Movement, EntityState, CellMoveState, RcEntity};
use game_scene::GameEvent;
use map::CellState;
use sdl2_engine_helpers::event_bus::EventBus;
use rock::{handle_collisions, think};

pub struct Crystal {
    momentum: bool,
}

impl Crystal {
    pub fn new() -> Box<EntityType> {
        Box::new(Crystal {
            momentum: false
        })
    }
}

impl EntityType for Crystal {
    fn input(&mut self, state: &mut EntityState, key: Movement, adjacents: &Adjacents) {
    }

    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
        handle_collisions(state, event_bus, cell_state)
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, tick: u64) {
        think(state, event_bus, adjacents, tick, &mut self.momentum);
    }

    fn is_hard(&self) -> bool {
        true
    }

    fn is_enterable(&self) -> bool {
        true
    }

    fn push(&mut self, direction: Movement, tick: u64) {
    }
}
