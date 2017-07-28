pub struct Audio {
}

impl Audio {
    pub fn new() -> Audio {
        // We are not using sublibraries so we don't need to initialize
        // let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::empty()).expect("Could not initialize Mixer");
        let frequency = 44100;
        let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        let channels = DEFAULT_CHANNELS; // Stereo
        let chunk_size = 1024;
        sdl2::mixer::open_audio(frequency, format, channels, chunk_size).expect("Failed to open Audio");
        sdl2::mixer::allocate_channels(4);

        let sound_chunk = sdl2::mixer::Chunk::from_file("./assets/explosion.wav").expect("Failed to load example sound");
        sdl2::mixer::Channel::all().play(&sound_chunk, 0).expect("Failed to play sound");
        Audio {
        }
    }
}
