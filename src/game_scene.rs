use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};
use sdl2_engine_helpers::keyhandler::KeyHandler;
use sdl2_engine_helpers::event_bus::EventBus;
use map::{Map, CELL_SIZE};
use entity::{Movement, RcEntity};
use game_data::GameData;

pub enum GameEvent {
    MoveRequest(Movement),
    Dig(u32, u32),
    Crushed(RcEntity),
    Explosion(u32, u32),
    Push(Movement, RcEntity),
    Collect(RcEntity),
}

pub struct GameScene {
    quitting: bool,
    map: Map,
    keyhandler: KeyHandler,
    camera_pos: (u32, u32),
}

impl GameScene {
    pub fn new<'a>(renderer: &mut Canvas<Window>) -> BoxedScene<Event, Canvas<Window>, GameData<'a>> {
        let mut scene = Box::new(GameScene {
            quitting: false,
            map: Map::new(50, 50),
            keyhandler: KeyHandler::new(),
            camera_pos: (0, 0),
        });
        scene.adjust_camera_position(renderer);
        scene
    }

    fn adjust_camera_position(&mut self, canvas: &Canvas<Window>) {
        let (old_x, old_y) = self.camera_pos;
        let (screen_width, screen_height) = canvas.output_size().expect("Could not get screen size");
        let min_x = (screen_width / 2) as i64;
        let min_y = (screen_height / 2) as i64;

        let player = self.map.player.borrow();

        let (target_x, target_y) = player.get_abs_position();
        let (target_x, target_y) = (target_x as i64 + (CELL_SIZE as i64 / 2) - min_x, target_y as i64 + (CELL_SIZE as i64 / 2) - min_y);
        let (adjusted_x, adjusted_y) = (
            if target_x < 0 { 0 } else { target_x },
            if target_y < 0 { 0 } else { target_y },
        );
        // Constrain if we'd be viewing out of bounds
        //
        /*if (target_x < min_x) {
            target_x = min_x;
        }
        if (target_y < min_y) {
            target_y = min_y;
        }*/

        self.camera_pos = (adjusted_x as u32, adjusted_y as u32);

    }

}

impl<'a> Scene<Event, Canvas<Window>, GameData<'a>> for GameScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &GameData, tick: u64) {
        self.map.render(renderer, engine_data, tick, self.camera_pos);
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Canvas<Window>, engine_data: &mut GameData, tick: u64) {
        match event {
            &Event::KeyDown {keycode: Some(Keycode::Escape), ..} => self.quitting = true,
            e => self.keyhandler.handle_event(e)
        }
    }

    fn think(&mut self, renderer: &mut Canvas<Window>, engine_data: &mut GameData, tick: u64) -> Option<SceneChangeEvent<Event, Canvas<Window>, GameData<'a>>> {
        if self.quitting {
            Some(SceneChangeEvent::PopScene)
        } else {
            let mut event_bus = EventBus::new();
            self.keyhandler.think(tick);
            // Temporary movement
            if self.keyhandler.is_pressed(Keycode::Left) {
                event_bus.enqueue(GameEvent::MoveRequest(Movement::LEFT));
            }
            if self.keyhandler.is_pressed(Keycode::Right) {
                event_bus.enqueue(GameEvent::MoveRequest(Movement::RIGHT));
            }
            if self.keyhandler.is_pressed(Keycode::Up) {
                event_bus.enqueue(GameEvent::MoveRequest(Movement::UP));
            }
            if self.keyhandler.is_pressed(Keycode::Down) {
                event_bus.enqueue(GameEvent::MoveRequest(Movement::DOWN));
            }

            self.map.think(&mut event_bus, renderer, engine_data, tick);
            self.adjust_camera_position(renderer);
            None
        }
    }
}
