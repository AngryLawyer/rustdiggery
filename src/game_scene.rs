use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};
use sdl2_engine_helpers::keyhandler::KeyHandler;
use sdl2_engine_helpers::event_bus::EventBus;
use map::{Map, CELL_SIZE};
use entity::Movement;

pub enum GameEvent {
    MoveEvent(Movement),
    DigEvent(u32, u32)
}

pub struct GameScene {
    quitting: bool,
    map: Map,
    keyhandler: KeyHandler,
    tick: u64,
    next_think: u64,
    camera_pos: (u32, u32),
}

impl GameScene {
    pub fn new(renderer: &mut Canvas<Window>) -> BoxedScene<Event, Canvas<Window>, ()> {
        let mut scene = Box::new(GameScene {
            quitting: false,
            map: Map::new(50, 50),
            keyhandler: KeyHandler::new(),
            tick: 0,
            next_think: 0,
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

        let (target_x, target_y) = (player.x as i64 * CELL_SIZE as i64 + (CELL_SIZE as i64 / 2) - min_x, player.y as i64 * CELL_SIZE as i64 + (CELL_SIZE as i64 / 2) - min_y);
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

impl Scene<Event, Canvas<Window>, ()> for GameScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64) {
        self.map.render(renderer, engine_data, tick, self.camera_pos);
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) {
        match event {
            &Event::KeyDown {keycode: Some(Keycode::Escape), ..} => self.quitting = true,
            e => self.keyhandler.handle_event(e)
        }
    }

    fn think(&mut self, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) -> Option<SceneChangeEvent<Event, Canvas<Window>, ()>> {
        if self.quitting {
            Some(SceneChangeEvent::PopScene)
        } else {
            let mut event_bus = EventBus::new();
            self.keyhandler.think(tick);
            // Temporary movement
            if self.keyhandler.is_pressed(Keycode::Left) {
                event_bus.enqueue(GameEvent::MoveEvent(Movement::LEFT));
            }
            if self.keyhandler.is_pressed(Keycode::Right) {
                event_bus.enqueue(GameEvent::MoveEvent(Movement::RIGHT));
            }
            if self.keyhandler.is_pressed(Keycode::Up) {
                event_bus.enqueue(GameEvent::MoveEvent(Movement::UP));
            }
            if self.keyhandler.is_pressed(Keycode::Down) {
                event_bus.enqueue(GameEvent::MoveEvent(Movement::DOWN));
            }

            self.map.think(&mut event_bus, renderer, engine_data, tick);
            self.adjust_camera_position(renderer);
            None
        }
    }
    /*fn think(&mut self, args: &UpdateArgs) -> Option<SceneCommand> {
        self.tick += (args.dt * 100000.0) as u64;
        //self.keyhandler.think(self.tick);

        if self.tick >= self.next_think {
            self.next_think += 10000;
            {
                self.map.world.update();
                //FIXME: Make this use weak references once we have them
                /*let entity = self.world.player.clone();
                let mut entity = entity.borrow_mut();
                //Move existing
                let x = entity.x;
                let y = entity.y;
                entity.think(self.tick, &self.world.adjacents(x, y));

                let x = entity.x;
                let y = entity.y;
                self.world.set_pos(x, y, CellState::Empty);

                //Handle player input
                match self.keyhandler.last_key() {
                    Some((key, tick)) => {
                        let difference = self.tick - tick;
                        if difference < 8000 || difference > 20000 {
                            let x = entity.x;
                            let y = entity.y;
                            entity.input(key, &self.world.adjacents(x, y));
                        }
                    },
                    None => ()
                }*/
            }

            //self.adjust_camera_position();
        }

        if self.quit {
            Some(SceneCommand::PopScene)
        } else {
            None
        }*/
}
