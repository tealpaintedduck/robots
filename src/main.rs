extern crate piston_window;
extern crate sprite;
extern crate find_folder;
extern crate ai_behavior;

use piston_window::*;
use sprite::*;
use std::rc::Rc;
use ai_behavior::{
    Action
};

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let mut scene = Scene::new();

    let tex = Rc::new(
        Texture::from_path(
            &mut window.factory,
            assets.join("robo.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
    let mut robo = Sprite::from_texture(tex);
    robo.set_position(360.0, 240.0);
    let id = scene.add_child(robo);

    while let Some(e) = window.next() {
        let mut movement = Action(MoveBy(0.0, 0.0, 0.0));
        match e {
            Input::Press(button) => {
                match button {
                    Button::Keyboard(key) => {
                        match key {
                            Key::Up => {
                                movement = Action(MoveBy(0.1, 0.0, -15.0));
                            }
                            Key::Right => {
                                movement = Action(MoveBy(0.1, 15.0, 0.0));
                            }
                            Key::Down => {
                                movement = Action(MoveBy(0.1, 0.0, 15.0));
                            }
                            Key::Left => {
                                movement = Action(MoveBy(0.1, -15.0, 0.0));
                            }
                            Key::Space => {
                                movement = Action(ToggleVisibility);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Input::Render(_) => {
                window.draw_2d(&e, |c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    scene.draw(c.transform, g);
                });
            }
            _ => {}
        }

        scene.event(&e);
        scene.run(id, &movement);
    }
}