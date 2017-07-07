use entity::{EntityType, RcEntity};
use map::{CELL_SIZE, CellState};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use transform::TransformContext;
use game_data::GameData;
use entity::EntityState;
use game_scene::GameEvent;
use sdl2_engine_helpers::event_bus::EventBus;

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
