mod cell;
mod piece;
mod game;

use game::Game;



fn main() {


    let mut game = Game::default();
    let result = game.play_cli();
    println!("{}", result);
    
}