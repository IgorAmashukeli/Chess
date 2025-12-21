use crate::{game::{Game, shapes::*, free::*}, piece::{self, Color, PieceType}};

pub fn is_correct_cell(row : u8, col : u8) -> bool {
    row <= 7 && col <= 7
}

pub fn is_correct_cell_i(row : i8, col : i8) -> bool {
    row >= 0 && row <= 7 && col >= 0 && col <= 7
}

pub fn cell_to_coord(row: u8, col: u8) -> String {
    static FILES: &[u8; 8] = b"abcdefgh";
    format!("{}{}", FILES[col as usize] as char, row + 1)
}

pub fn is_correct_piece_ind(piece_ind : u8) -> bool {
    return piece_ind < 32;
}

pub fn get_piece_ind(game : &Game, row : u8, col : u8) -> Option<u8> {
    assert!(is_correct_cell(row, col));

    return game.board[row as usize][col as usize].piece;
}

pub fn reachable_start(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> u8 {
    assert!(is_correct_cell(row_st, col_st));
    assert!(is_correct_cell(row_fn, col_fn));
    let piece_ind_opt = get_piece_ind(game, row_st, col_st);
    assert!(piece_ind_opt != None);
    let piece_ind = piece_ind_opt.unwrap();
    assert!(is_correct_piece_ind(piece_ind));
    return piece_ind;
}

pub fn get_piece_color(game : &Game, piece_ind : u8) -> Color {
    return game.piece_set[piece_ind as usize].color;
}

pub fn get_piece_type(game : &Game, piece_ind : u8) -> PieceType {
    return game.piece_set[piece_ind as usize].piece_type
}



pub fn is_reachable_knight(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(get_piece_type(game, piece_ind) == PieceType::Knight);
    return is_free_or_opponent(game, row_fn, col_fn, get_piece_color(game, piece_ind)) && 
    is_knight_shape(row_st, col_st, row_fn, col_fn);
}


pub fn is_reachable_bishop(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(get_piece_type(game, piece_ind) == PieceType::Bishop);
    return is_free_or_opponent(game, row_fn, col_fn, get_piece_color(game, piece_ind)) && 
    is_bishop_shape(row_st, col_st, row_fn, col_fn) && is_free_on_path_bishop(game, row_st, col_st, row_fn, col_fn);
}


pub fn is_reachable_rook(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(get_piece_type(game, piece_ind) == PieceType::Rook);
    return is_free_or_opponent(game, row_fn, col_fn, get_piece_color(game, piece_ind)) && 
    is_rook_shape(row_st, col_st, row_fn, col_fn) && is_free_on_path_rook(game, row_st, col_st, row_fn, col_fn);
}


pub fn is_reachable_queen(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(get_piece_type(game, piece_ind) == PieceType::Queen);
    return is_free_or_opponent(game, row_fn, col_fn, get_piece_color(game, piece_ind)) &&
    is_queen_shape(row_st, col_st, row_fn, col_fn) &&is_free_on_path_queen(game, row_st, col_st, row_fn, col_fn);
}


pub fn is_reachable_king(game :&Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(get_piece_type(game, piece_ind) == PieceType::King);
    if is_king_shape(row_st, col_st, row_fn, col_fn) {
        return is_free_or_opponent(game, row_fn, col_fn, get_piece_color(game, piece_ind));
    }

    if is_king_wh_shrt_castle_shape(row_st, col_st, row_fn, col_fn) || is_king_wh_lng_castle_shape(row_st, col_st, row_fn, col_fn) {
        return get_piece_color(game, piece_ind) == Color::White && is_free_on_path_castling(game, row_st, col_st, col_fn);
    }

    if is_king_blck_shrt_castle_shape(row_st, col_st, row_fn, col_fn) || is_king_blck_lng_castle_shape(row_st, col_st, row_fn, col_fn) {

        return get_piece_color(game, piece_ind) == Color::Black && is_free_on_path_castling(game, row_st, col_st, col_fn);
    }

    return false;
}


pub fn is_reachable_pawn(game :&Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let piece_ind = reachable_start(game, row_st, col_st, row_fn, col_fn);
    assert!(get_piece_type(game, piece_ind) == PieceType::Pawn);
    if is_pawn_shape_move_wht(row_st, col_st, row_fn, col_fn) {
        let piece_color = get_piece_color(game, piece_ind);
        return piece_color == piece::Color::White && is_free(game, row_fn, col_fn) && 
        is_free_on_path_move_pawn(game, row_st, col_st, row_fn);
    }

    if is_pawn_shape_take_wht(row_st, col_st, row_fn, col_fn) {
        let piece_color = get_piece_color(game, piece_ind);
        if is_opponent(game, row_fn, col_fn, piece_color) {
            return true;
        }

        if let Some(near_piece_ind) = get_piece_ind(game, row_st, col_fn) {

            return is_free(game, row_fn, col_fn) && is_pawn_shape_en_passant_wht(row_st, col_st, row_fn, col_fn) 
            && get_piece_color(game, near_piece_ind) == Color::Black && get_piece_type(game, near_piece_ind) == PieceType::Pawn;
        }

        return false;
        
    }

    if is_pawn_shape_move_blck(row_st, col_st, row_fn, col_fn) {
        let piece_color = get_piece_color(game, piece_ind);
        return piece_color == Color::Black && is_free(game, row_fn, col_fn) && 
        is_free_on_path_move_pawn(game, row_st, col_st, row_fn);
    }

    if is_pawn_shape_take_blck(row_st, col_st, row_fn, col_fn) {
        let piece_color = get_piece_color(game, piece_ind);
        if is_opponent(game, row_fn, col_fn, piece_color) {
            return true;
        }

        if let Some(near_piece_ind) = get_piece_ind(game, row_st, col_fn) {

            return is_free(game, row_fn, col_fn) && is_pawn_shape_en_passant_blck(row_st, col_st, row_fn, col_fn) 
            && get_piece_color(game, near_piece_ind) == Color::White && get_piece_type(game, near_piece_ind) == PieceType::Pawn;
        }

        return false;
    }

    return false;

}


pub fn is_reachable_color(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    assert!(is_correct_cell(row_st, col_st));
    assert!(is_correct_cell(row_fn, col_fn));

    if let Some(piece_ind) = get_piece_ind(game, row_st, col_st) {

        let piece_type = get_piece_type(game, piece_ind);
        match piece_type {
            PieceType::Knight => return is_reachable_knight(game, row_st, col_st, row_fn, col_fn),
            PieceType::Bishop => return is_reachable_bishop(game, row_st, col_st, row_fn, col_fn),
            PieceType::Rook => return is_reachable_rook(game, row_st, col_st, row_fn, col_fn),
            PieceType::Queen => return is_reachable_queen(game, row_st, col_st, row_fn, col_fn),
            PieceType::King => return is_reachable_king(game, row_st, col_st, row_fn, col_fn),
            PieceType::Pawn => return is_reachable_pawn(game, row_st, col_st, row_fn, col_fn),
        }
    }

    return false;
}


pub fn is_reachable(game :&Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    assert!(is_correct_cell(row_st, col_st));
    assert!(is_correct_cell(row_fn, col_fn));


    if let Some(piece_ind) = get_piece_ind(game, row_st, col_st) {
        if get_piece_color(game, piece_ind) != game.active_color {
            return false;
        }

        let piece_type = get_piece_type(game, piece_ind);
        match piece_type {
            PieceType::Knight => return is_reachable_knight(game, row_st, col_st, row_fn, col_fn),
            PieceType::Bishop => return is_reachable_bishop(game, row_st, col_st, row_fn, col_fn),
            PieceType::Rook => return is_reachable_rook(game, row_st, col_st, row_fn, col_fn),
            PieceType::Queen => return is_reachable_queen(game, row_st, col_st, row_fn, col_fn),
            PieceType::King => return is_reachable_king(game, row_st, col_st, row_fn, col_fn),
            PieceType::Pawn => return is_reachable_pawn(game, row_st, col_st, row_fn, col_fn),
        }
    }

    return false;
}