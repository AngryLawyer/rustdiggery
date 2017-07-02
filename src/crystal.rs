use entity::{EntityType, EntityState, RcEntity};
use game_scene::GameEvent;
use map::CellState;
use map::{CELL_SIZE, Adjacents};
use rock::{handle_collisions, think};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::event_bus::EventBus;
use transform::TransformContext;
use game_data::GameData;

pub struct Crystal {
    momentum: bool,
}

impl Crystal {
    pub fn new() -> Box<EntityType> {
        Box::new(Crystal {
            momentum: false,
        })
    }
}

impl EntityType for Crystal {
    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
        handle_collisions(state, event_bus, cell_state, self.momentum);
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, engine_data: &GameData, tick: u64) {
        think(state, event_bus, adjacents, tick, &mut self.momentum);
        if tick % 60 == 0 {
            state.animation_state.advance(&engine_data.animations.crystal);
        }
    }

    fn is_hard(&self) -> bool {
        true
    }

    fn is_enterable(&self) -> bool {
        true
    }

    fn is_collectible(&self) -> bool {
        true
    }

    fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, state: &EntityState, engine_data: &GameData, tick: u64) {
        let image = &engine_data.assets.crystal;
        let source = state.animation_state.get_frame(&engine_data.animations.crystal).expect("Could not get animation frame");
        transform.copy(renderer, image, source.source_bounds, Rect::new(0, 0, CELL_SIZE, CELL_SIZE)).expect("Failed to draw entity");
    }

    fn score(&self) -> u32 {
        1
    }
}
