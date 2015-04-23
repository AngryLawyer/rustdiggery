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
use opengl_graphics::GlGraphics;

use std::cell::RefCell;
use std::rc::Rc;

pub mod scene;
pub mod scene_manager;
pub mod title_scene;
pub mod game_scene;
pub mod entity;
pub mod keyhandler;

fn main() {

    let opengl = shader_version::opengl::OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Rustdiggery".to_string(),
            window::Size {width: 800, height: 600},
        ).exit_on_esc(true)
    );
    let window = Rc::new(RefCell::new(window));

    let mut manager = scene_manager::SceneManager::new();
    let mut gl = GlGraphics::new(opengl);
    manager.push_scene(title_scene::TitleScene::new());
    
    for e in window.events() {
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
                manager.think(&u);
            } else {
                break;
            }
        }
    }
}
