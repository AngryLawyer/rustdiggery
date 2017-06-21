use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use assets::Assets;

pub struct GameData {
    assets: Assets
}

impl GameData {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> GameData {
        GameData {
            assets: Assets::new(texture_creator)
        }
    }
}
