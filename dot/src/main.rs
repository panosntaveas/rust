extern crate piston_window;
use piston_window::PistonWindow;

mod draw;
use draw::{clear_screen, get_window, color, Col::White};

mod game;
mod terrain;
mod ball;
use game::Game;

fn main() {

    let mut game= Game::new(600, 400, 100, 100);
    let mut window: PistonWindow = get_window("Snake", game.terrain.width as u32, game.terrain.height as u32);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear_screen(g);
            game.terrain.draw(color(White), c, g);
            game.ball.draw(color(White), c, g);
        }); 
        game.update();
    }
}
