use sdl2_engine_helpers::scene::{BoxedScene, Scene, SceneChangeEvent};

pub struct TitleScene;

impl TitleScene {
    pub fn new() -> BoxedScene<(), (), ()> {
        Box::new(TitleScene)
    }
}

impl Scene<(), (), ()> for TitleScene {

    fn render(&self, renderer: &mut (), engine_data: &mut (), tick: u64) {
    }

    fn handle_event(&mut self, event: &(), renderer: &mut (), engine_data: &mut (), tick: u64) -> Option<SceneChangeEvent<(), (), ()>> {
        None
    }
}
