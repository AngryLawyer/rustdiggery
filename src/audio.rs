use std::collections::HashMap;
use sdl2::mixer::{Chunk, Channel, open_audio, allocate_channels};
use sdl2::mixer::{DEFAULT_CHANNELS, AUDIO_S16LSB};

pub struct Audio {
    sounds: HashMap<String, Chunk>
}

impl Audio {
    pub fn new() -> Audio {
        // We are not using sublibraries so we don't need to initialize
        // let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::empty()).expect("Could not initialize Mixer");
        let frequency = 44100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1024;
        open_audio(frequency, format, channels, chunk_size).expect("Failed to open Audio");
        allocate_channels(4);
        Audio {
            sounds: HashMap::new()
        }
    }

    pub fn load_sound(&mut self, path: &str) {
        let sound_chunk = Chunk::from_file(path).expect("Failed to load sound");
        self.sounds.insert(path.to_owned(), sound_chunk);
    }

    pub fn load_sounds(&mut self, paths: &[&str]) {
        for path in paths {
            self.load_sound(path);
        }
    }

    pub fn play_sound(&self, path: &str) {
        match self.sounds.get(path) {
            Some(chunk) => {
                Channel::all().play(chunk, 0).expect("Failed to play sound");
            },
            None => ()
        }
    }
}
