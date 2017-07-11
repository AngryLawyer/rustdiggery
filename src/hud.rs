use game_data::GameData;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Hud {
}

impl Hud {
    pub fn new() -> Hud {
        Hud {}
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, engine_data: &GameData, tick: u64) {
        let (screen_width, screen_height) = renderer.output_size().expect("Could not get screen size");
        let bar_height = 48;
        renderer.set_draw_color(Color::RGB(50, 100, 100));
        renderer.fill_rect(Rect::new(0, (screen_height - bar_height) as i32, screen_width, bar_height)).expect("Could not draw UI");
    }
}
