use bitmap_font::Font;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Assets<'a> {
    pub font: Font<'a>,
    pub crystal: Texture<'a>,
    pub tileset_raw: Texture<'a>,
    pub explosion: Texture<'a>,
}

impl<'a> Assets<'a> {
    fn load_font(font_texture: Texture<'a>) -> Font<'a> {
        let mut font_letters = [None; 256];
        let font_size = 32;
        let offset_from_top = 6 * font_size;
        for i in 0..(16 * 6) {
            let target = 32 + i;
            let bounds = Some(Rect::new((i % 16) * font_size, ((i / 16) * font_size) + offset_from_top, font_size as u32, font_size as u32));
            font_letters[target as usize] = bounds;
        }
        Font::new(font_texture, font_letters)
    }

    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Assets {
        let crystal = texture_creator.load_texture("assets/crystal.png").expect("Could not load assets/crystal.png");

        let font_texture = texture_creator.load_texture("assets/jeromBD-metal3-CCBY3.png").expect("Could not load assets/jeromBD-metal3-CCBY3.png");
        let font = Assets::load_font(font_texture);

        let tileset_raw = texture_creator.load_texture("assets/dirt-tiles.png").expect("Could not load assets/dirt-tiles.png");
        let explosion = texture_creator.load_texture("assets/M484ExplosionSet1.png").expect("Could not load assets/M484ExplosionSet1.png");

        Assets {
            explosion,
            font,
            crystal,
            tileset_raw,
        }
    }
}
