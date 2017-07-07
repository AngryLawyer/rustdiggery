use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

pub struct Assets<'a> {
    pub crystal: Texture<'a>
}

impl<'a> Assets<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Assets {
        let texture = texture_creator.load_texture("assets/crystal.png").expect("Could not load assets/crystal.png");
        Assets {
            crystal: texture
        }
    }
}