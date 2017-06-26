use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use assets::Assets;

pub struct GameData<'a> {
    assets: Assets<'a>
}

impl<'a> GameData<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> GameData {
        GameData {
            assets: Assets::new(texture_creator)
        }
    }
}
