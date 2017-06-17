use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};
use game_scene::GameScene;
use game_data::GameData;

pub struct TitleScene {
    quitting: bool,
    continuing: bool
}

impl TitleScene {
    pub fn new() -> BoxedScene<Event, Canvas<Window>, GameData> {
        Box::new(TitleScene {
            quitting: false,
            continuing: false
        })
    }
}

impl Scene<Event, Canvas<Window>, GameData> for TitleScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &GameData, tick: u64) {
        let color = (tick % 256) as u8;
        renderer.set_draw_color(Color::RGB(color, color, color));
        renderer.clear();
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Canvas<Window>, engine_data: &mut GameData, tick: u64) {
        match *event {
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => self.quitting = true,
            Event::KeyDown {..} => self.continuing = true,
            _ => ()
        }
    }

    fn think(&mut self, renderer: &mut Canvas<Window>, engine_data: &mut GameData, tick: u64) -> Option<SceneChangeEvent<Event, Canvas<Window>, GameData>> {
        if self.quitting {
            Some(SceneChangeEvent::PopScene)
        } else if self.continuing {
            self.continuing = false;
            Some(SceneChangeEvent::PushScene(Box::new(|renderer, _| { GameScene::new(renderer) })))
        } else {
            None
        }
    }
}
