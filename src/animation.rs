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
}

struct AnimationFrame {
}
