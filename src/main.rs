extern crate sdl2;
extern crate sdl2_engine_helpers;

pub mod title_scene;
pub mod game_scene;
pub mod entity;
pub mod map;
pub mod transform;
pub mod player;
pub mod rock;

use sdl2_engine_helpers::game_loop::GameLoop;
use sdl2_engine_helpers::scene::SceneStack;
use sdl2::event::Event;

use title_scene::TitleScene;

fn main() {
    let sdl_context = sdl2::init().expect("Could not initialize SDL context");
    let video_subsystem = sdl_context.video().expect("Could not initialize video subsystem");
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
                        scene_stack.handle_event(&event, &mut canvas, &mut (), frame_number);
                    }
                }
            };
            scene_stack.think(&mut canvas, &mut (), frame_number);
            scene_stack.render(&mut canvas, &(), frame_number);
            false
        }
    });
}
