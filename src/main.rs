#![feature(alloc)];
extern crate sdl2_window;
extern crate window;
extern crate shader_version;
extern crate event;
extern crate input;
extern crate opengl_graphics;
extern crate graphics;

use sdl2_window::Sdl2Window as Window;
use window::WindowSettings;
use event::{Events, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent};
use opengl_graphics::Gl;

use std::cell::RefCell;

pub mod scene;
pub mod scene_manager;
pub mod title_scene;
pub mod game_scene;
pub mod entity;

fn main() {

    let opengl = shader_version::opengl::OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings {
            title: "Rustdiggery".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: false,
            samples: 4,
        }
    );
    let window = RefCell::new(window);

    let mut manager = scene_manager::SceneManager::new();
    let mut gl = Gl::new(opengl);
    manager.push_scene(title_scene::TitleScene::new());
    
    for e in event::events(&window) {
        if let Some(p) = e.press_args() {
            manager.press(&p);
        }

        if let Some(p) = e.release_args() {
            manager.release(&p);
        }

        if let Some(r) = e.render_args() {
            manager.render(&mut gl, &r);
        }
        if let Some(u) = e.update_args() {
            //app.update(&mut *window.borrow_mut(), &u);
            if manager.scene_count() > 0 {
                manager.think();
            } else {
                break;
            }
        }
    }
}
