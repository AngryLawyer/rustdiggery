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

    pub fn get_frame(&self, animation: &str, frame: usize) -> Option<&AnimationFrame> {
        match self.animations.get(animation) {
            Some(frames) => {
                frames.get(frame)
            },
            None => None
        }
    }

    pub fn get_animation(&self, animation: &str) -> Option<&Animation> {
        self.animations.get(animation)
    }
}

pub struct Animation {
    frames: Vec<AnimationFrame>
}

impl Animation {
    pub fn new(frames: Vec<AnimationFrame>) -> Animation {
        Animation {
            frames: frames
        }
    }

    pub fn add(&mut self, frame: AnimationFrame) {
        self.frames.push(frame);
    }

    pub fn get(&self, idx: usize) -> Option<&AnimationFrame> {
        self.frames.get(idx)
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

pub struct AnimationFrame {
    pub source_bounds: Rect
}

impl AnimationFrame {
    pub fn new(source: &Rect) -> AnimationFrame {
        AnimationFrame {
            source_bounds: source.clone()
        }
    }
}

pub struct AnimationState {
    // TODO: Find a nice way to make this deal in references
    current_animation: String,
    current_frame: usize,
}

impl AnimationState {
    pub fn new() -> AnimationState {
        AnimationState {
            current_animation: "idle".to_owned(),
            current_frame: 0,
        }
    }

    pub fn get_frame<'a>(&self, animation: &'a AnimationSet) -> Option<&'a AnimationFrame> {
        animation.get_frame(&self.current_animation, self.current_frame)
    }

    pub fn advance(&mut self, animation: &AnimationSet) {
        // TODO: Currently we assume all animations just loop.
        match animation.get_animation(&self.current_animation) {
            Some(frames) => {
                if self.current_frame == frames.len() - 1{
                    self.current_frame = 0;
                } else {
                    self.current_frame += 1;
                }
            },
            None => ()
        }
    }
}
