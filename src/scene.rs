use sdl2::event::Event;
use sdl2::render::Canvas;
use game_data::GameData;
use sdl2_engine_helpers::scene::{BoxedScene, Scene};
use sdl2::video::Window;

pub type RustdiggeryScene<'a> =  Scene<Event, Canvas<Window>, GameData<'a>, SceneChange>;
pub type BoxedRustdiggeryScene<'a> = BoxedScene<Event, Canvas<Window>, GameData<'a>, SceneChange>;

pub enum SceneChange {
    InterstitalScene,
    GameScene
}
