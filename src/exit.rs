use cell_state::CellState;
use entity::EntityState;
use entity::{EntityType, RcEntity};
use game_data::GameData;
use game_scene::GameEvent;
use map::CELL_SIZE;
use map::{Adjacents};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::event_bus::EventBus;
use transform::TransformContext;

pub struct Exit {
    open: bool,
}

impl Exit{
    pub fn new() -> Box<EntityType> {
        Box::new(Exit{
            open: false
        })
    }
}

impl EntityType for Exit {
    fn is_enterable(&self) -> bool {
        self.open
    }

    fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, state: &EntityState, engine_data: &GameData, tick: u64) {
        let image = &engine_data.assets.tileset_raw;
        let source = state.animation_state.get_frame(&engine_data.animations.exit).expect("Could not get animation frame");
        transform.copy(renderer, image, source.source_bounds, Rect::new(0, 0, CELL_SIZE, CELL_SIZE)).expect("Failed to draw entity");
    }

    fn think(&mut self, state: &mut EntityState, event_bus: &mut EventBus<GameEvent>, adjacents: &Adjacents, engine_data: &GameData, tick: u64) {
        if tick % 5 == 0 {
            state.animation_state.advance(&engine_data.animations.exit);
        }
    }

    fn open_exit(&mut self, state: &mut EntityState) {
        if self.open == false {
            self.open = true;
            state.animation_state.set_animation("open");
        }
    }

    fn destructable(&self) -> bool {
        false
    }

    fn collisions(&self, state: &EntityState, event_bus: &mut EventBus<GameEvent>, cell_state: (CellState, Vec<RcEntity>)) {
        match cell_state {
            (_, ref items) if items.len() > 0 => {
                let item = items.first().unwrap();
                if item.borrow().is_player() {
                    event_bus.enqueue(GameEvent::Complete);
                }
            },
            _ => ()
        }
    }
}
