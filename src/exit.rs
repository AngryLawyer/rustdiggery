use entity::{EntityType};
use map::{CELL_SIZE};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use transform::TransformContext;
use game_data::GameData;

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

    fn render(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, engine_data: &GameData, tick: u64) {
        if self.open {
            renderer.set_draw_color(Color::RGB(255, 0, 255));
        } else {
            renderer.set_draw_color(Color::RGB(255, 255, 255));
        }
        transform.fill_rect(
            renderer,
            Rect::new(
                0,
                0,
                CELL_SIZE,
                CELL_SIZE
            )
        ).expect("Failed to draw entity");
    }

    fn open_exit(&mut self) {
        self.open = true;
    }

    fn destructable(&self) -> bool {
        false
    }
}
