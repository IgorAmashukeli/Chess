use crate::{game::{Game, free::*, generate::* , reachable::*, shapes::* }, piece::*};
use std::{collections::HashMap};

static WHITE_KING_IND : u8 = 15;
static BLACK_KING_IND : u8 = 31;
static WHITE_ROOK_QUEEN_IND : u8 = 12;
static WHITE_ROOK_KING_IND : u8 = 13;
static BLACK_ROOK_QUEEN_IND : u8 = 28;
static BLACK_ROOK_KING_IND : u8 = 29;


pub fn is_under_check(game : &Game) -> bool {
    let cell;
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
        game.black_short_castle, game.black_long_castle, game.enpassant_sq, game.rule50_clock, game.move_number, HashMap::new());
    try_to_move(&mut game_after_move, row_st, col_st, row_fn, col_fn);
    return is_under_check(&game_after_move);
}


pub fn move_piece(game : &mut Game, piece_ind : u8, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) {
    game.board[row_st as usize][col_st as usize].piece = None;
    game.board[row_fn as usize][col_fn as usize].piece = Some(piece_ind);
    game.piece_set[piece_ind as usize].cell = Some((row_fn, col_fn));
}


pub fn take_piece(game : &mut Game, row : u8, col : u8) {
    let piece_ind_opt = get_piece_ind(game, row, col);
    assert!(piece_ind_opt != None);
    let piece_ind = piece_ind_opt.unwrap();
    game.piece_set[piece_ind as usize].cell = None;
    game.board[row as usize][col as usize].piece = None;
}


pub fn try_to_move(game : &mut Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) {
    let piece_ind_opt = get_piece_ind(game, row_st, col_st);
    assert!(piece_ind_opt != None);
    let piece_ind = piece_ind_opt.unwrap();
    
    if is_free(game, row_fn, col_fn) {
        if get_piece_type(game, piece_ind) == PieceType::Pawn && 
        (is_pawn_shape_en_passant_wht(row_st, col_st, row_fn, col_fn) || is_pawn_shape_en_passant_blck(row_st, col_st, row_fn, col_fn)) {
            assert!(is_opponent(game, row_st, col_fn, game.active_color));
            take_piece(game, row_st, col_fn);
        }
    } else {
        assert!(is_opponent(game, row_fn, col_fn, game.active_color));
        take_piece(game, row_fn, col_fn);
    }
    move_piece(game, piece_ind, row_st, col_st, row_fn, col_fn);
}


pub fn is_king_and_between_under_check(game : &Game, row_st : u8, col_st : u8, col_fn : u8) -> bool {
    if is_under_check(game) {
        return true;
    }
    if col_st < col_fn {
        for i in 1..3 {
            if is_after_move_under_check(game, row_st, col_st, row_st, col_st + i) {
                return true;
            }
        }
    } else {
        for i in 1..4 {
            if is_after_move_under_check(game, row_st, col_st, row_st, col_st - i) {
                return true;
            }
        }
    }
    

    return false;
}


pub fn can_move(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    if !is_correct_cell(row_st, col_st) || !is_correct_cell(row_fn, col_fn) {
        return false;
    }


    if !is_reachable(game, row_st, col_st, row_fn, col_fn) {
        return false;
    }

    let piece_ind_opt = get_piece_ind(game, row_st, col_st);
    assert!(piece_ind_opt != None);
    let piece_ind = piece_ind_opt.unwrap();

    if is_after_move_under_check(game, row_st, col_st, row_fn, col_fn) {
        return false;
    }

    let piece_type = get_piece_type(game, piece_ind);

    if (piece_type == PieceType::Pawn) && is_pawn_shape_en_passant_wht(row_st, col_st, row_fn, col_fn) && 
    is_free(game, row_fn, col_fn) {
        if game.enpassant_sq == None {
            return false;
        }

        let en_pass_sq = game.enpassant_sq.unwrap();
        return (en_pass_sq.0 == row_st) && (en_pass_sq.1 == col_fn);
    }


    if (piece_type == PieceType::Pawn) && is_pawn_shape_en_passant_blck(row_st, col_st, row_fn, col_fn) && 
    is_free(game, row_fn, col_fn) {

        if game.enpassant_sq == None {
            return false;
        }

        let en_pass_sq = game.enpassant_sq.unwrap();
        return (en_pass_sq.0 == row_st) && (en_pass_sq.1 == col_fn);
    }


    if piece_type == PieceType::King && is_king_wh_shrt_castle_shape(row_st, col_st, row_fn, col_fn) {
        if !game.white_short_castle {
            return false;
        }

        return !is_king_and_between_under_check(game, row_st, col_st, col_fn);

    }

    if piece_type == PieceType::King && is_king_wh_lng_castle_shape(row_st, col_st, row_fn, col_fn) {
        if !game.white_long_castle {
            return false;
        }

        return !is_king_and_between_under_check(game, row_st, col_st, col_fn);
    }

    if piece_type == PieceType::King && is_king_blck_shrt_castle_shape(row_st, col_st, row_fn, col_fn) {
        if !game.black_short_castle {
            return false;
        }

        return !is_king_and_between_under_check(game, row_st, col_st, col_fn);
    }

    if piece_type == PieceType::King && is_king_blck_lng_castle_shape(row_st, col_st, row_fn, col_fn) {
        if !game.black_long_castle {
            return false;
        }

        return !is_king_and_between_under_check(game, row_st, col_st, col_fn);
    }

    return true;
}





pub fn gen_all_piece_moves(game : &Game, row_st : u8, col_st : u8, piece_type : PieceType,
     res : &mut Vec<String>) {
     match piece_type {
            PieceType::Knight => gen_all_moves_knight(game, row_st, col_st, res),

            PieceType::Bishop => gen_all_moves_bishop(game, row_st, col_st, res),

            PieceType::Rook => gen_all_moves_rook(game, row_st, col_st, res),

            PieceType::Queen => gen_all_moves_queen(game, row_st, col_st, res),

            PieceType::King => gen_all_moves_king(game, row_st, col_st, res),

            PieceType::Pawn => gen_all_moves_pawn(game, row_st, col_st, res)
        }
}


pub fn gen_all_moves(game : &Game) -> Vec<String> {
    let mut res : Vec<String> = [].to_vec();

    for i in 0..16 {
        let mut ind = i;
        if game.active_color == Color::Black {
            ind = i + 16;
        }
        let piece_cell_opt = game.piece_set[ind as usize].cell;
        if piece_cell_opt == None {
            continue;
        }
        let piece_cell = piece_cell_opt.unwrap();

        gen_all_piece_moves(game, piece_cell.0, piece_cell.1, get_piece_type(game, ind), &mut res);
    }

    return res;
}

pub fn add_offers(moves : &mut Vec<String>, was_draw : bool) {
    if !was_draw {
        moves.push("Select the move and offer a draw".to_string());
    } else  {
        moves.push("Accept the draw".to_string());
    }
    moves.push("Resign".to_string());
}


pub fn change_castling_flags(game : &mut Game, piece_ind : u8) {
    let piece_type = get_piece_type(game, piece_ind);
    let color = get_piece_color(game, piece_ind);
    if (piece_type == PieceType::King) && (color == Color::White)
    {
        game.white_short_castle = false;
        game.white_long_castle = false;
    }
    if (piece_type == PieceType::King) && (color == Color::Black) {
        game.black_short_castle = false;
        game.black_long_castle = false;
    }

    if piece_ind == 12 {
        game.white_long_castle = false;
    }
    if piece_ind == 13 {
        game.white_long_castle = false;
    }
    if piece_ind == 28 {
        game.black_long_castle = false;
    }
    if piece_ind == 29 {
        game.black_short_castle = false;
    }
}


pub fn change_en_passant_flag(game : &mut Game, row_st : u8, row_fn : u8, col_fn : u8, piece_ind : u8) {
    let piece_type = get_piece_type(game, piece_ind);
    let color = get_piece_color(game, piece_ind);
    if (piece_type == PieceType::Pawn) && (color == Color::White) && (row_fn - row_st == 2) {
        game.enpassant_sq = Some((row_fn, col_fn));
    } else if (piece_type == PieceType::Pawn) && (color == Color::Black) && (row_st - row_fn == 2) {
        game.enpassant_sq = Some((row_fn, col_fn));
    } else {
        game.enpassant_sq = None;
    }
}


pub fn make_move(game : &mut Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8, promotion : Option<PieceType>) {
    let piece_ind_opt = get_piece_ind(game, row_st, col_st);
    assert!(piece_ind_opt != None);
    let piece_ind = piece_ind_opt.unwrap();

    if is_free(game, row_fn, col_fn) {

        

        if get_piece_type(game, piece_ind) == PieceType::Pawn && 
        (is_pawn_shape_en_passant_wht(row_st, col_st, row_fn, col_fn) || is_pawn_shape_en_passant_blck(row_st, col_st, row_fn, col_fn)) {
            assert!(is_opponent(game, row_st, col_fn, game.active_color));
            take_piece(game, row_st, col_fn);
        }

        change_castling_flags(game, piece_ind);
        change_en_passant_flag(game, row_st, row_fn, col_fn, piece_ind);

        game.rule50_clock = if get_piece_type(game, piece_ind) == PieceType::Pawn {1} else {game.rule50_clock + 1};
    } else {
        assert!(is_opponent(game, row_fn, col_fn, game.active_color));
        take_piece(game, row_fn, col_fn);

        game.rule50_clock = 0;
    }
    move_piece(game, piece_ind, row_st, col_st, row_fn, col_fn);

    if get_piece_type(game, piece_ind) == PieceType::King && is_king_wh_shrt_castle_shape(row_st, col_st, row_fn, col_fn) {
        move_piece(game, WHITE_ROOK_KING_IND, 0, 7, 0, 5);
    } else if get_piece_type(game, piece_ind) == PieceType::King && is_king_wh_lng_castle_shape(row_st, col_st, row_fn, col_fn) {
        move_piece(game, WHITE_ROOK_QUEEN_IND, 0, 0, 0, 3);
    } else if get_piece_type(game, piece_ind) == PieceType::King && is_king_blck_shrt_castle_shape(row_st, col_st, row_fn, col_fn) {
        move_piece(game, BLACK_ROOK_KING_IND, 7, 7, 7, 5);
    } else if get_piece_type(game, piece_ind) == PieceType::King && is_king_blck_lng_castle_shape(row_st, col_st, row_fn, col_fn) {
        move_piece(game, BLACK_ROOK_QUEEN_IND, 7, 0, 7, 3);
    }

    if let Some(promotion_type) = promotion {
        game.piece_set[piece_ind as usize].piece_type = promotion_type;
    }

    game.move_number += if game.active_color == Color::White {0} else {1}; 
    game.active_color = if game.active_color == Color::White {Color::Black} else {Color::White};

}










