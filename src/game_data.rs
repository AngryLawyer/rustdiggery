use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use assets::Assets;
use audio::Audio;
use animation::{AnimationSet, Animation, AnimationFrame};
use map::{CELL_SIZE};
use map_loader::{load_maps, MapData};
use tileset::Tileset;

pub struct Animations {
    pub crystal: AnimationSet,
    pub exit: AnimationSet,
    pub explosion: AnimationSet,
}

pub struct GameData<'a> {
    pub assets: &'a Assets<'a>,
    pub animations: Animations,
    pub maps: Vec<MapData>,
    pub tileset: Tileset<'a>,
    pub audio: Audio,
}

impl<'a> GameData<'a> {
    fn crystal_animation() -> AnimationSet {
        let mut crystal = AnimationSet::new();
        let frame_1 = AnimationFrame::new(&Rect::new(CELL_SIZE as i32, 0, CELL_SIZE, CELL_SIZE));
        let frame_2 = AnimationFrame::new(&Rect::new(0, CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        let anim = Animation::new(vec![frame_1, frame_2]);
        crystal.add("idle", anim);
        crystal
    }

    fn exit_animation() -> AnimationSet {
        let mut exit = AnimationSet::new();
        let frame_1 = AnimationFrame::new(&Rect::new(5 * CELL_SIZE as i32, 4 * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        let frame_2 = AnimationFrame::new(&Rect::new(4 * CELL_SIZE as i32, 4 * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        let frame_3 = AnimationFrame::new(&Rect::new(5 * CELL_SIZE as i32, 5 * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        let frame_4 = AnimationFrame::new(&Rect::new(4 * CELL_SIZE as i32, 5 * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        let frame_5 = AnimationFrame::new(&Rect::new(5 * CELL_SIZE as i32, 5 * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        let frame_6 = AnimationFrame::new(&Rect::new(4 * CELL_SIZE as i32, 4 * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        exit.add("idle", Animation::new(vec![frame_1.clone()]));
        exit.add("open", Animation::new(vec![frame_1, frame_2, frame_3, frame_4, frame_5, frame_6]));
        exit
    }

    fn explosion_animation() -> AnimationSet {
        let mut explosion = AnimationSet::new();
        let frame_1 = AnimationFrame::new(&Rect::new(92, 10, CELL_SIZE, CELL_SIZE));
        let frame_2 = AnimationFrame::new(&Rect::new(92 + ((CELL_SIZE as i32 + 2) * 1), 10, CELL_SIZE, CELL_SIZE));
        let frame_3 = AnimationFrame::new(&Rect::new(92 + ((CELL_SIZE as i32 + 2) * 2), 10, CELL_SIZE, CELL_SIZE));
        let frame_4 = AnimationFrame::new(&Rect::new(92 + ((CELL_SIZE as i32 + 2) * 3), 10, CELL_SIZE, CELL_SIZE));
        let frame_5 = AnimationFrame::new(&Rect::new(92 + ((CELL_SIZE as i32 + 2) * 4), 10, CELL_SIZE, CELL_SIZE));
        let frame_6 = AnimationFrame::new(&Rect::new(92 + ((CELL_SIZE as i32 + 2) * 5), 10, CELL_SIZE, CELL_SIZE));
        let frame_7 = AnimationFrame::new(&Rect::new(92 + ((CELL_SIZE as i32 + 2) * 6), 10, CELL_SIZE, CELL_SIZE));
        explosion.add("idle", Animation::new(vec![frame_1, frame_2, frame_3, frame_4, frame_5, frame_6, frame_7]));
        explosion
    }

    fn load_audio() -> Audio {
        let mut audio = Audio::new();
        audio.load_sounds(&[
            "assets/crystal.wav",
            "assets/dig.wav",
            "assets/explosion.wav",
        ]);
        audio
    }

    pub fn new(assets: &'a Assets) -> GameData<'a> {
        let crystal = GameData::crystal_animation();
        let exit = GameData::exit_animation();

        let maps = load_maps().expect("Failed to load maps");

        let tileset = Tileset::new(&assets.tileset_raw, 32);

        GameData {
            assets,
            audio: GameData::load_audio(),
            animations: Animations {
                crystal: crystal,
                exit: exit,
                explosion: GameData::explosion_animation(),
            },
            maps: maps,
            tileset,
        }
    }
}
