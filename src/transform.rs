use sdl2::render::{Canvas, RenderTarget};
use sdl2::rect::Rect;

pub struct TransformContext {
    x: i32,
    y: i32
}

impl TransformContext {
    pub fn new() -> TransformContext {
        TransformContext {
            x: 0,
            y: 0
        }
    }

    pub fn transform(&self, x: i32, y: i32) -> TransformContext {
        TransformContext {
            x: self.x + x,
            y: self.y + y
        }
    }

    pub fn fill_rect<R: Into<Option<Rect>>, T: RenderTarget>(&self, canvas: &mut Canvas<T>, rect: R) -> Result<(), String> {
        match rect.into() {
            Some(rect) => {
                let updated = Rect::new(rect.x + self.x, rect.y + self.y, rect.w as u32, rect.h as u32);
                canvas.fill_rect(Some(updated))
            },
            None => canvas.fill_rect(None)
        }
    }
}
