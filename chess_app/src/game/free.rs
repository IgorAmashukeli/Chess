use crate::{game::{Game, reachable::{is_correct_cell}}, piece::Color};
use std::cmp;

pub fn is_free(game : &Game, row : u8, col : u8) -> bool {
    game.board[row as usize][col as usize].is_free()
}


pub fn is_opponent(game : &Game, row : u8, col : u8, piece_color : Color) -> bool {
    let piece_ind_opt = game.board[row as usize][col as usize].piece;
    if let Some(piece_ind) = piece_ind_opt {
        return game.piece_set[piece_ind as usize].color != piece_color
    }
    return false
}


pub fn is_free_or_opponent(game : &Game, row : u8, col : u8, piece_color : Color) -> bool {
    let piece_ind_opt = game.board[row as usize][col as usize].piece;
    if let Some(piece_ind) = piece_ind_opt {
        return game.piece_set[piece_ind as usize].color != piece_color
    }
    return true
}


pub fn is_main_dir_bishop(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return ((row_st < row_fn) && (col_st < col_fn)) || ((row_st > row_fn) && (col_st > col_fn));
}


pub fn is_free_on_path_bishop(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    for i in 1..dist_row {
        let row_ind = cmp::min(row_st, row_fn) + i;
        let mut col_ind = cmp::min(col_st, col_fn) + i;
        if !is_main_dir_bishop(row_st, col_st, row_fn, col_fn) {
            col_ind = cmp::max(col_st, col_fn) - i;
        }
        assert!(is_correct_cell(row_ind, col_ind));
        if !is_free(game, row_ind, col_ind) {
            return false;
        }
    
    }

    return true;
}


pub fn is_free_on_path_rook(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    let dist_col = cmp::max(col_st, col_fn) - cmp::min(col_st, col_fn);

    if dist_row == 0 {
        for i in 1..dist_col {
            let row_ind = row_st;
            let col_ind = cmp::min(col_st, col_fn) + i;
            assert!(is_correct_cell(row_ind, col_ind));
            if !is_free(game, row_ind, col_ind) {
                return false;
            }
        }
        return true;
    } else {
        for i in 1..dist_row {
            let row_ind = cmp::min(row_st, row_fn) + i;
            let col_ind = col_st;
            assert!(is_correct_cell(row_ind, col_ind));
            if !is_free(game, row_ind, col_ind) {
                return false;
            }
        }
        return true;
    }
}


pub fn is_free_on_path_queen(game : &Game, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    let dist_col = cmp::max(col_st, col_fn) - cmp::min(col_st, col_fn);
    if dist_row == dist_col {
        return is_free_on_path_bishop(game, row_st, col_st, row_fn, col_fn);
    }

    assert!(dist_row == 0 || dist_col == 0);
    return is_free_on_path_rook(game, row_st, col_st, row_fn, col_fn)
}


pub fn is_free_on_path_castling(game : &Game, row_st : u8, col_st : u8, col_fn : u8) -> bool {
    

    if col_st < col_fn {
        for i in 1..3 {
            let row_ind = row_st;
            let col_ind = col_st + i;
            if !is_free(game, row_ind, col_ind) {
                return false;
            }
        }
    } else {
        for i in 1..4 {
            let row_ind = row_st;
            let col_ind = col_st - i;
            if !is_free(game, row_ind, col_ind) {
                
                return false;
            }
        }
    }
    return true;
}


pub fn is_free_on_path_move_pawn(game : &Game, row_st : u8, col_st : u8, row_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    if dist_row == 2 {
        return is_free(game, cmp::min(row_st, row_fn) + 1, col_st);
    }
    return true
}


