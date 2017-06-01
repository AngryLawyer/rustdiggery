use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};
use map::Map;

pub struct GameScene {
    quit: bool,
    map: Map,
    //keyhandler: KeyHandler,
    tick: u64,
    next_think: u64,
    camera_pos: (f64, f64)
}

impl GameScene {
    pub fn new() -> BoxedScene<Event, Canvas<Window>, ()> {
        Box::new(GameScene {
            quit: false,
            map: Map::new(10, 10),
            //keyhandler: KeyHandler::new(),
            tick: 0,
            next_think: 0,
            camera_pos: (0.0, 0.0)
        })
    }
}

impl Scene<Event, Canvas<Window>, ()> for GameScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64) {
        self.map.render(renderer, engine_data, tick);
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) -> Option<SceneChangeEvent<Event, Canvas<Window>, ()>> {
        match *event {
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => Some(SceneChangeEvent::PopScene),
            _ => None
        }
    }

    /*fn adjust_camera_position(&mut self) {
        let (old_x, old_y) = self.camera_pos;
        let player = self.world.player.borrow();
        self.camera_pos = ((player.x * 32)  as f64, (player.y * 32) as f64);
    }*/

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
