use crate::{game::{Game, reachable::*, shapes::*}, piece::*};
use crate::{game::shapes};

static WHITE_KING_IND : u8 = 15;
static BLACK_KING_IND : u8 = 31;


pub fn is_under_check(game : &Game) -> bool {
    let mut cell;
    if game.active_color == Color::White {
        let wh_kn = game.piece_set[WHITE_KING_IND as usize].cell;
        assert!(wh_kn != None);
        cell = wh_kn.unwrap();
    } else {
        let bl_kn = game.piece_set[BLACK_KING_IND as usize].cell;
        assert!(bl_kn != None);
        cell = bl_kn.unwrap();
    }

    for i in 0..16 {
        let mut ind = i;
        if game.active_color == Color::White {
            ind = i + 16;
        }
        let piece_cell_opt = game.piece_set[ind as usize].cell;
        if piece_cell_opt == None {
            continue;
        }
        let piece_cell = piece_cell_opt.unwrap();
        if is_reachable_color(game, piece_cell.0, piece_cell.1, cell.0, cell.1) {
            return true;
        }
    }
    return false;
}



pub fn is_after_move_under_check(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let mut game_after_move = Game::new(game.board, game.piece_set, game.active_color, game.white_short_castle, game.white_long_castle, 
        game.black_short_castle, game.black_long_castle, game.enpassant_sq, game.rule50_clock, game.move_number);
    try_move(&mut game_after_move, row_st, col_st, row_fn, col_fn);
    return is_under_check(&game_after_move);
}


pub fn try_move(game : &mut Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) {

}



pub fn can_move(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    if !is_reachable(game, row_st, col_st, row_fn, col_fn) {
        return false;
    }

    let piece_ind_opt = get_piece_ind(game, row_st, col_st);
    assert!(piece_ind_opt != None);
    let piece_ind = piece_ind_opt.unwrap();


    if is_after_move_under_check(game, row_st, row_fn, col_st, col_fn) {
        return false;
    }

    
    // TODO : check en passant and castling rules
   


    return true;
}


pub fn make_move(game : &mut Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) {

}


pub fn gen_all_moves(game : &Game) {

}







