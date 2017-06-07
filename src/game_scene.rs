use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};
use map::{Map, CELL_SIZE};

pub struct GameScene {
    quitting: bool,
    map: Map,
    //keyhandler: KeyHandler,
    tick: u64,
    next_think: u64,
    camera_pos: (u32, u32)
}

impl GameScene {
    pub fn new() -> BoxedScene<Event, Canvas<Window>, ()> {
        Box::new(GameScene {
            quitting: false,
            map: Map::new(10, 10),
            //keyhandler: KeyHandler::new(),
            tick: 0,
            next_think: 0,
            camera_pos: (0, 0)
        })
    }

    fn adjust_camera_position(&mut self, canvas: &Canvas<Window>) {
        let (old_x, old_y) = self.camera_pos;
        let (screen_width, screen_height) = canvas.output_size().expect("Could not get screen size");
        let min_x = screen_width / 2;
        let min_y = screen_height / 2;

        let player = self.map.player.borrow();

        let (target_x, target_y) = (player.x as i64 * CELL_SIZE as i64 + (CELL_SIZE as i64 / 2) - min_x as i64, player.y as i64 * CELL_SIZE as i64 + (CELL_SIZE as i64 / 2) - min_y as i64);
        // Constrain if we'd be viewing out of bounds
        //
        /*if (target_x < min_x) {
            target_x = min_x;
        }
        if (target_y < min_y) {
            target_y = min_y;
        }*/

        self.camera_pos = (target_x as u32, target_y as u32);

    }

}

impl Scene<Event, Canvas<Window>, ()> for GameScene {

    fn render(&self, renderer: &mut Canvas<Window>, engine_data: &(), tick: u64) {
        self.map.render(renderer, engine_data, tick, self.camera_pos);
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) {
        match *event {
            Event::KeyDown {keycode: Some(Keycode::Escape), ..} => self.quitting = true,
            _ => ()
        }
    }

    fn think(&mut self, renderer: &mut Canvas<Window>, engine_data: &mut (), tick: u64) -> Option<SceneChangeEvent<Event, Canvas<Window>, ()>> {
        if self.quitting {
            Some(SceneChangeEvent::PopScene)
        } else {
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
