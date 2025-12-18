mod cell;
mod piece;
mod game;

use game::Game;
use game::moves;



fn main() {


    let my_game = game::Game::default();
    my_game.play();

    
}