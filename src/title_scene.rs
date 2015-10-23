use scene::{Scene, BoxedScene, SceneCommand};
use piston_window::*;

pub struct TitleScene;

impl TitleScene {
    pub fn new() -> BoxedScene {
        Box::new(TitleScene)
    }
}

impl Scene for TitleScene {

    fn handle_event(&mut self, e: &PistonWindow) -> Option<SceneCommand> {
        e.draw_2d(|_c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
        None
    }
}
