extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;

use game::Game;
use draw::to_coord_u32;
use piston_window::*;
use piston_window::types::Color;

const BLACK_COLOR : Color = [0.5, 0.5, 0.5, 1.0];
const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

fn main() {
    let mut window : PistonWindow = WindowSettings::new("Snake", [to_coord_u32(WIDTH), to_coord_u32(HEIGHT)])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game::new(WIDTH, HEIGHT);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(k)) = event.press_args() {
            game.key_press(k);
        }
        window.draw_2d(&event, |c, g| {
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|e| {
            game.update(e.dt);
        });
    }
}