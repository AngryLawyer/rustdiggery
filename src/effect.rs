use animation::{AnimationState, AnimationSet};
use game_data::GameData;
use map::{CELL_SIZE, Adjacents};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use transform::TransformContext;
use std::rc::Rc;


#[derive(Clone)]
pub struct Effect {
    pub x: u32,
    pub y: u32,
    pub animation_state: AnimationState,
    pub done: bool,
}


impl Effect {
    pub fn new(animation: Rc<AnimationSet>, x: u32, y: u32) -> Effect {
        Effect {
            x,
            y,
            animation_state: AnimationState::new(animation),
            done: false
        }
    }

    pub fn think(&mut self, engine_data: &GameData, tick: u64) {
        if tick % 5 == 0 {
            self.animation_state.advance();
        }
        if self.animation_state.is_last_frame() {
            self.done = true;
        }
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, engine_data: &GameData, tick: u64) {
        let image = &engine_data.assets.explosion;
        let source = self.animation_state.get_frame().expect("Could not get animation frame");
        transform.copy(renderer, image, source.source_bounds, Rect::new(0, 0, CELL_SIZE, CELL_SIZE)).expect("Failed to draw entity");
    }
}
