use sdl2::rect::Rect;
use std::collections::HashMap;

pub struct AnimationSet {
    animations: HashMap<String, Animation>,
}

impl AnimationSet {
    pub fn new() -> AnimationSet {
        AnimationSet {
            animations: HashMap::new()
        }
    }

    pub fn add<T: Into<String>>(&mut self, name: T, animation: Animation) {
        self.animations.insert(name.into(), animation);
    }
}

pub struct Animation {
    frames: Vec<AnimationFrame>
}

impl Animation {
    pub fn new() -> Animation {
        Animation {
            frames: vec![]
        }
    }

    pub fn add(&mut self, frame: AnimationFrame) {
        self.frames.push(frame);
    }
}

pub struct AnimationFrame {
    source_bounds: Rect
}

impl AnimationFrame {
    pub fn new(source: &Rect) -> AnimationFrame {
        AnimationFrame {
            source_bounds: source.clone()
        }
    }
}
