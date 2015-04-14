use std::collections::HashMap;
use input::Key;

enum KeyState {
    Press,
    Release 
}

pub struct KeyHandler {
    incoming: Vec<(KeyState, Key)>,
    last_press: Option<Key>,
    keys: HashMap<Key, u64>
}

impl KeyHandler {
    pub fn new() -> KeyHandler {
        KeyHandler {
            incoming: vec![],
            last_press: None,
            keys: HashMap::new()
        }
    }

    pub fn press(&mut self, key: Key) {
        self.incoming.push((KeyState::Press, key));
    }

    pub fn release(&mut self, key: Key) {
        self.incoming.push((KeyState::Release, key));
    }

    pub fn think(&mut self, tick: u64) {
        // Digest each of the incoming items;
        for &(ref state, ref key) in self.incoming.iter() {
            match *state {
                KeyState::Press => {
                    self.last_press = Some(*key);
                    self.keys.insert(*key, tick);
                },
                KeyState::Release => {
                    match self.last_press {
                        Some(last_key) if last_key == *key => {
                            self.last_press = None
                        },
                        _ => ()
                    };
                    self.keys.remove(key);
                }
            }
        };
        self.incoming.clear();
    }

    pub fn last_key(&self) -> Option<(Key, u64)> {
        match self.last_press {
            Some(key) => {
                match self.keys.get(&key) {
                    Some(timestamp) => {
                        Some((key, *timestamp))
                    },
                    None => None
                }
            },
            None => None
        }
    }
}
