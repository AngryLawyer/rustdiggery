extern crate piston_window;
#[macro_use]
extern crate ecs;

use piston_window::*;

pub mod scene;
pub mod scene_manager;
pub mod title_scene;
pub mod game_scene;
//pub mod entity;
//pub mod keyhandler;

fn main() {

    let opengl = OpenGL::V3_2;
    let (width, height) = (800, 600);
    let window: PistonWindow =
        WindowSettings::new("Rustdigery", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut manager = scene_manager::SceneManager::new();
    manager.push_scene(game_scene::GameScene::new());

    for e in window {
        manager.handle_event(&e);
        if let Some(_u) = e.update_args() {
            if manager.scene_count() == 0 {
                break;
            }
        }
    }
}
