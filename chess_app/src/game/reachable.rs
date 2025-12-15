use crate::{game::{Game, shapes::*, free::*}, piece::{self, Color}};

pub fn is_correct_cell(row : u8, col : u8) -> bool {
    row <= 7 && col <= 7
}


pub fn reachable_start(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> u8 {
    assert!(is_correct_cell(row_st, col_st));
    assert!(is_correct_cell(row_fn, col_fn));
    let piece_ind_opt = game.board[row_st as usize][col_st as usize].piece;
    assert!(piece_ind_opt != None);
    return piece_ind_opt.unwrap();
}


pub fn is_reachable_knight(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(game.piece_set[piece_ind as usize].piece_type == piece::PieceType::Knight);
    return is_free_or_opponent(game, row_fn, col_fn, game.piece_set[piece_ind as usize].color) && 
    is_knight_shape(row_st, col_st, row_fn, col_fn);
}


pub fn is_reachable_bishop(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(game.piece_set[piece_ind as usize].piece_type == piece::PieceType::Bishop);
    return is_free_or_opponent(game, row_fn, col_fn, game.piece_set[piece_ind as usize].color) && 
    is_bishop_shape(row_st, col_st, row_fn, col_fn) && is_free_on_path_bishop(game, row_st, col_st, row_fn, col_fn);
}


pub fn is_reachable_rook(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    return is_free_or_opponent(game, row_fn, col_fn, game.piece_set[piece_ind as usize].color) && 
    is_rook_shape(row_st, col_st, row_fn, col_fn) && is_free_on_path_rook(game, row_st, col_st, row_fn, col_fn);
}