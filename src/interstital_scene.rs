use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};
use game_scene::GameScene;
use game_data::GameData;
use map_loader::MapData;

pub struct InterstitalScene {
    continuing: bool
}

impl InterstitalScene {
    pub fn new<'a>() -> BoxedScene<Event, Canvas<Window>, GameData<'a>> {
        Box::new(InterstitalScene {
            continuing: false
        })
    }
}

impl<'a> Scene<Event, Canvas<Window>, GameData<'a>> for InterstitalScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &GameData, tick: u64) {
        let color = (tick % 256) as u8;
        renderer.set_draw_color(Color::RGB(color, 0, 0));
        renderer.clear();
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Canvas<Window>, engine_data: &mut GameData, tick: u64) {
        match *event {
            Event::KeyDown {..} => self.continuing = true,
            _ => ()
        }
    }

    fn think(&mut self, renderer: &mut Canvas<Window>, engine_data: &mut GameData, tick: u64) -> Option<SceneChangeEvent<Event, Canvas<Window>, GameData<'a>>> {
        if self.continuing {
            self.continuing = false;
            //let map: MapData = engine_data.maps[0].clone();
            let game_scene = GameScene::new(renderer, engine_data.maps[0].clone());
            Some(SceneChangeEvent::SwapScene(Box::new(|renderer, _| { game_scene })))
        } else {
            None
        }
    }
}
