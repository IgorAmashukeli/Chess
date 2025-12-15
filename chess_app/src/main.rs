mod cell;
mod piece;
mod game;

use game::Game;
use game::moves;



fn main() {
    let my_cel = cell::Cell::new(Some(2));
    println!("{:?}", my_cel);

    let my_piece = piece::Piece::new(piece::Color::White, piece::PieceType::King, None);
    println!("{:?}", my_piece);

    let my_game = game::Game::default();
    my_game.play();

    
}