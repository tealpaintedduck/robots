extern crate piston_window;
extern crate sprite;
extern crate find_folder;
extern crate ai_behavior;

use piston_window::*;
use sprite::*;
use std::rc::Rc;
use ai_behavior::{
    Action,
    Sequence
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

    let robo_tex = Rc::new(
        Texture::from_path(
            &mut window.factory,
            assets.join("robo-main.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());

    let side_tex = Rc::new(
        Texture::from_path(
            &mut window.factory,
            assets.join("robo-side.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());

    let front_tex = Rc::new(
        Texture::from_path(
            &mut window.factory,
            assets.join("robo.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap());

    let mut robo = Sprite::from_texture(robo_tex);
    let mut side_robo = Sprite::from_texture(side_tex);
    let mut front_robo = Sprite::from_texture(front_tex);
    side_robo.set_visible(false);
    let side_robo_id = robo.add_child(side_robo);
    let front_robo_id = robo.add_child(front_robo);
    robo.set_position(360.0, 240.0);
    let id = scene.add_child(robo);

    while let Some(e) = window.next() {
        let mut robo_movement = Action(MoveBy(0.0, 0.0, 0.0));
        let mut side_visibility = Action(MoveBy(0.0, 0.0, 0.0));
        let mut front_visibility = Action(MoveBy(0.0, 0.0, 0.0));
        match e {
            Input::Press(button) => {
                match button {
                    Button::Keyboard(key) => {
                        match key {
                            Key::Up => {
                                robo_movement = Action(MoveBy(0.1, 0.0, -15.0));
                                side_visibility = Action(Hide);
                                front_visibility = Action(Show);
                            }
                            Key::Right => {
                                robo_movement = Action(MoveBy(0.1, 15.0, 0.0));
                                side_visibility = Sequence(vec![
                                                    Action(Show),
                                                    Action(FlipX(false))]);
                                front_visibility = Action(Hide);
                            }
                            Key::Down => {
                                robo_movement = Action(MoveBy(0.1, 0.0, 15.0));
                                side_visibility = Action(Hide);
                                front_visibility = Action(Show);
                            }
                            Key::Left => {
                                robo_movement = Action(MoveBy(0.1, -15.0, 0.0));
                                side_visibility = Sequence(vec![
                                                    Action(Show),
                                                    Action(FlipX(true))]);
                                front_visibility = Action(Hide);
                            }
                            Key::Space => {
                                robo_movement = Action(ToggleVisibility);
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
        scene.run(id, &robo_movement);
        scene.run(front_robo_id, &front_visibility);
        scene.run(side_robo_id, &side_visibility);
    }
}