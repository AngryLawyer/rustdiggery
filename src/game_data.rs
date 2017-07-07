use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use assets::Assets;
use animation::{AnimationSet, Animation, AnimationFrame};
use map::{CELL_SIZE};
use map_loader::{load_maps, MapData};

pub struct Animations {
    pub crystal: AnimationSet
}

pub struct GameData<'a> {
    pub assets: Assets<'a>,
    pub animations: Animations,
    pub maps: Vec<MapData>,
}

impl<'a> GameData<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> GameData {
        let mut crystal = AnimationSet::new();
        let frame_1 = AnimationFrame::new(&Rect::new(CELL_SIZE as i32, 0, CELL_SIZE, CELL_SIZE));
        let frame_2 = AnimationFrame::new(&Rect::new(0, CELL_SIZE as i32, CELL_SIZE, CELL_SIZE));
        let anim = Animation::new(vec![frame_1, frame_2]);
        crystal.add("idle", anim);

        let maps = load_maps().expect("Failed to load maps");

        GameData {
            assets: Assets::new(texture_creator),
            animations: Animations {
                crystal: crystal
            },
            maps: maps,
        }
    }
}
