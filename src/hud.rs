use game_data::GameData;
use map::Map;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use transform::TransformContext;

pub struct Hud {
}

impl Hud {
    pub fn new() -> Hud {
        Hud {}
    }

    pub fn render(&self, renderer: &mut Canvas<Window>, map: &Map, engine_data: &GameData, tick: u64) {
        let (screen_width, screen_height) = renderer.output_size().expect("Could not get screen size");
        let bar_height = 48;
        let transform = TransformContext::new()
            .transform(0, (screen_height - bar_height) as i32);
        renderer.set_draw_color(Color::RGB(50, 100, 100));
        transform.fill_rect(renderer, Rect::new(0, 0, screen_width, bar_height)).expect("Could not draw UI");

        engine_data.assets.font.blit_text(renderer, &transform.transform(12, 8), &format!("{}/{}", map.crystals_collected, map.crystals_to_pass));
    }
}
