mod piece;
mod cell;
mod game;

use crate::game::Game;



fn main() {
    let mut game = Game::default();

    let ans = game.play_cli();

    println!("{}. {}", ans.1, ans.0);
}
