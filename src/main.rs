extern crate piston_window;

use piston_window::*;

/*pub mod scene;
pub mod scene_manager;
pub mod title_scene;
pub mod game_scene;
pub mod entity;
pub mod keyhandler;*/

fn main() {

    let opengl = OpenGL::V3_2;
    let (width, height) = (800, 600);
    let window: PistonWindow =
        WindowSettings::new("Rustdigery", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    //let window = Rc::new(RefCell::new(window));
    //let ref mut gl = GlGraphics::new(opengl);

    //let mut manager = scene_manager::SceneManager::new();
    //manager.push_scene(title_scene::TitleScene::new());
    
    for e in window {
        if let Some(button) = e.press_args() {
            //manager.press(&p);
        }

        if let Some(button) = e.release_args() {
            //manager.release(&p);
        }
    }
    /*for e in window.events() {
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
    }*/
}
