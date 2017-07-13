use sdl2::image::LoadTexture;
use sdl2::rect::{Rect, Point};
use sdl2::render::Texture;
use transform::TransformContext;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum FlipContext {
    FlipNone,
    FlipHorizontal,
    FlipVertical,
    FlipBoth
}

pub struct Tileset<'a> {
    texture: Texture<'a>,
    tile_size: u32
}

impl<'a> Tileset<'a> {
    pub fn new(texture: Texture<'a>, tile_size: u32) -> Tileset<'a> {
        Tileset {
            texture,
            tile_size
        }
    }

    pub fn blit_sprite(&self, renderer: &mut Canvas<Window>, x: u32, y: u32, transform: &TransformContext, flip_context: Option<FlipContext>) {
        let source_bounds = Rect::new((x * self.tile_size) as i32, (y * self.tile_size) as i32, self.tile_size, self.tile_size);
        let (flip_h, flip_v) = match flip_context {
            None | Some(FlipContext::FlipNone) => (false, false),
            Some(FlipContext::FlipHorizontal) => (true, false),
            Some(FlipContext::FlipVertical) => (false, true),
            Some(FlipContext::FlipBoth) => (true, true)
        };
        transform.copy_ex(
            renderer,
            &self.texture,
            source_bounds,
            Rect::new(0, 0, self.tile_size, self.tile_size),
            0.0,
            None,
            flip_h,
            flip_v
        ).expect("Failed to draw sprite");
    }
}
