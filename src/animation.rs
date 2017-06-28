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

    pub fn get_frame(&self, animation: &str, frame: usize) -> Option<(&AnimationFrame, usize)> {
        match self.animations.get(animation) {
            Some(frames) => {
                frames.get(frame)
            },
            None => None
        }
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

    pub fn get(&self, idx: usize) -> Option<(&AnimationFrame, usize)> {
        match self.frames.get(idx) {
            Some(frame) => {
                if idx == self.frames.len() - 1 {
                    Some((frame, 0))
                } else {
                    Some((frame, idx + 1))
                }
            }
        }
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
