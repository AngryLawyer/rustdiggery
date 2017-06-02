use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};
use game_scene::GameScene;

pub struct TitleScene {
    quitting: bool,
    continuing: bool
}

impl TitleScene {
    pub fn new() -> BoxedScene<Event, Canvas<Window>, ()> {
        Box::new(TitleScene {
            quitting: false,
            continuing: false
        })
    }
}

impl Scene<Event, Canvas<Window>, ()> for TitleScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64) {
        let color = (tick % 256) as u8;
        renderer.set_draw_color(Color::RGB(color, color, color));
        renderer.clear();
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) {
        match *event {
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => self.quitting = true,
            Event::KeyDown {..} => self.continuing = true,
            _ => ()
        }
    }

    fn think(&mut self, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) -> Option<SceneChangeEvent<Event, Canvas<Window>, ()>> {
        if self.quitting {
            Some(SceneChangeEvent::PopScene)
        } else if self.continuing {
            self.continuing = false;
            Some(SceneChangeEvent::PushScene(Box::new(|_, _| { GameScene::new() })))
        } else {
            None
        }
    }
}
