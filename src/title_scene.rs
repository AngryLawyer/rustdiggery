use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};

pub struct TitleScene;

impl TitleScene {
    pub fn new() -> BoxedScene<(), Canvas<Window>, ()> {
        Box::new(TitleScene)
    }
}

impl Scene<(), Canvas<Window>, ()> for TitleScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) {
        let color = (tick % 256) as u8;
        renderer.set_draw_color(Color::RGB(color, color, color));
        renderer.clear();
        renderer.present();
    }

    fn handle_event(&mut self, event: &(), renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) -> Option<SceneChangeEvent<(), Canvas<Window>, ()>> {
        None
    }
}
