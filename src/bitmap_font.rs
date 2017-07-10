use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use transform::TransformContext;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Font<'a> {
    texture: Texture<'a>,
    letters: [Option<Rect>; 256],
}

impl<'a> Font<'a> {
    pub fn new(texture: Texture<'a>, letters: [Option<Rect>; 256]) -> Font<'a> {
        Font {
            texture,
            letters
        }
    }

    //TODO: We probably want a way of producing fixed textures too
    pub fn blit_text(&self, renderer: &mut Canvas<Window>, transform: &TransformContext, text: &str) {
        let mut transform = transform.clone();
        for c in text.chars() {
            let index = c as u8;
            match self.letters[index as usize] {
                Some(bounds) => {
                    transform.copy(renderer, &self.texture, bounds, Rect::new(0, 0, bounds.width(), bounds.height())).expect("Failed to draw font");
                    transform = transform.transform(bounds.width() as i32, 0);
                },
                None => ()
            }
        }
    }
}
