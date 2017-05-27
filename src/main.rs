extern crate sdl2;
extern crate sdl2_engine_helpers;

//pub mod scene;
//pub mod scene_manager;
pub mod title_scene;
//pub mod game_scene;
//pub mod entity;
//pub mod keyhandler;
use sdl2_engine_helpers::game_loop::GameLoop;
use sdl2_engine_helpers::scene::SceneStack;
use sdl2::pixels::Color;

use title_scene::TitleScene;

fn main() {
    let sdl_context = sdl2::init().expect("Could not initialize SDL context");
    let video_subsystem = sdl_context.video().expect("Could not initialize video subsystem");
    let window = video_subsystem.window("RustDiggery", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Could not build window");

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .expect("Could not get Canvas");

    let mut scene_stack = SceneStack::new();
    scene_stack.push(TitleScene::new());

    let mut game_loop = GameLoop::new(60);
    game_loop.run(|frame_number| {
        if scene_stack.is_empty() {
            true
        } else {
            let color = (frame_number % 256) as u8;
            canvas.set_draw_color(Color::RGB(color, color, color));
            canvas.clear();
            canvas.present();
            false
        }
    });
}
