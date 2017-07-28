extern crate sdl2;
extern crate sdl2_engine_helpers;
extern crate serde;
extern crate serde_json;


#[macro_use]
extern crate serde_derive;

pub mod animation;
pub mod assets;
pub mod crystal;
pub mod enemy;
pub mod entity;
pub mod exit;
pub mod game_data;
pub mod game_scene;
pub mod interstital_scene;
pub mod map;
pub mod player;
pub mod rock;
pub mod title_scene;
pub mod transform;
pub mod map_loader;
pub mod scene;
pub mod bitmap_font;
pub mod hud;
pub mod tileset;
pub mod cell_state;
pub mod effect;

use sdl2_engine_helpers::game_loop::GameLoop;
use sdl2_engine_helpers::scene::SceneStack;
use sdl2::event::Event;
use sdl2::image::{INIT_PNG};
use sdl2::mixer::{DEFAULT_CHANNELS, AUDIO_S16LSB};

use title_scene::TitleScene;
use interstital_scene::InterstitalScene;
use game_scene::GameScene;
use game_data::GameData;
use scene::SceneChange;
use assets::Assets;

fn main() {
    let sdl_context = sdl2::init().expect("Could not initialize SDL context");
    let video_subsystem = sdl_context.video().expect("Could not initialize video subsystem");
    let _image_context = sdl2::image::init(INIT_PNG).expect("Could not initialize SDL_Image context");
    // We are not using sublibraries so we don't need to initialize
    let _audio = sdl_context.audio().expect("Could not initialize SDL Audio");

    let window = video_subsystem.window("RustDiggery", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not build window");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Could not get Canvas");

    let texture_creator = canvas.texture_creator();
    let assets = Assets::new(&texture_creator);
    let mut game_data = GameData::new(&assets);

    let mut scene_stack = SceneStack::new();
    scene_stack.push(TitleScene::new());

    let mut event_pump = sdl_context.event_pump().expect("Could not fetch event pump");

    let mut game_loop = GameLoop::new(60);
    game_loop.run(|frame_number| {
        if scene_stack.is_empty() {
            true
        } else {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        return true
                    },
                    _ => {
                        scene_stack.handle_event(&event, &mut canvas, &mut game_data, frame_number);
                    }
                }
            };
            scene_stack.think(&mut canvas, &mut game_data, frame_number, &|event_data, renderer, game_data| {
                match *event_data {
                    SceneChange::InterstitalScene => {
                        InterstitalScene::new()
                    },
                    SceneChange::GameScene => {
                        GameScene::new(renderer, game_data.maps.get(0).unwrap())
                    },
                }
            });
            scene_stack.render(&mut canvas, &game_data, frame_number);
            false
        }
    });
}
