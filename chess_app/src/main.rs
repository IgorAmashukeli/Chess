mod cell;
mod piece;
mod game;

use game::Game;
use game::moves;



fn main() {


    let mut my_game = game::Game::default();
    let result = my_game.play();
    println!("{}", result);
    
}