use crate::{game::{Game, reachable::*, moves::*}, piece::*};


pub fn add_move(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8, res : &mut Vec<String>) {
    let cell_fn = cell_to_coord(row_fn, col_fn);
    res.push(cell_to_coord(row_st, col_st) + " " + cell_fn.as_str());
}


pub fn gen_all_moves_knight(game : &Game, row_st : u8, col_st : u8,
     res : &mut Vec<String>) {
    let dx : [i8; 2] = [-1, 1];
    let dy : [i8; 2] = [-2, 2];
    for i in dx {
        for j in dy {
            let mut row_ind = row_st as i8 + i;
            let mut col_ind = col_st as i8 + j;
            if is_correct_cell_i(row_ind, col_ind) {
                let row_fn = row_ind as u8;
                let col_fn = col_ind as u8;
                if can_move(game, row_st, col_st, row_fn, col_fn) {
                    add_move(row_st, col_st, row_fn, col_fn, res);
                }
            }
            
            
            row_ind = row_st as i8 + j;
            col_ind = col_st as i8 + i;
             if is_correct_cell_i(row_ind, col_ind) {
                let row_fn = row_ind as u8;
                let col_fn = col_ind as u8;
                if can_move(game, row_st, col_st, row_fn, col_fn) {
                    add_move(row_st, col_st, row_fn, col_fn, res);
                }
            }

        }
    }

}

pub fn gen_all_moves_bishop(game : &Game, row_st : u8, col_st : u8,
     res : &mut Vec<String>) {
    let dx : [i8; 2] = [-1, 1];
    for i in dx {
        for j in dx {
            let mut row_fn = row_st as i8 + i;
            let mut col_fn = col_st as i8 + j;
            while is_correct_cell_i(row_fn, col_fn) {
                if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                    add_move(row_st, col_st, row_fn as u8, col_fn as u8 , res);
                }
                row_fn += i;
                col_fn += j;
            }
        }
    }
}


pub fn gen_all_moves_rook(game : &Game, row_st : u8, col_st : u8,
     res : &mut Vec<String>) {
    for i in 0..8 {
        if can_move(game, row_st, col_st, row_st, i) {
            add_move(row_st, col_st, row_st, i , res);
        }
        if can_move(game, row_st, col_st, i, col_st) {
            add_move(row_st, col_st, i, col_st, res);
        }
    }
}

pub fn gen_all_moves_queen(game : &Game, row_st : u8, col_st : u8,
     res : &mut Vec<String>) {
    gen_all_moves_bishop(game, row_st, col_st, res);
    gen_all_moves_rook(game, row_st, col_st, res);
}

pub fn gen_all_moves_king(game : &Game, row_st : u8, col_st : u8,
     res : &mut Vec<String>) {
    let dx : [i8; 3] = [-1, 0, 1];
    for i in dx {
        for j in dx {
            let row_fn = row_st as i8 + i;
            let col_fn = col_st as i8 + i;
            if (i != 0 || j != 0) && is_correct_cell_i(row_fn, col_fn) {
                if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                    add_move(row_st, col_st, row_fn as u8, col_fn as u8, res);
                }
            }
        }
    }

    assert!(get_piece_ind(game, row_st, col_st) != None);


    if (row_st == 0) && (col_st == 4) && 
    (get_piece_color(game, get_piece_ind(game, row_st, col_st).unwrap()) == Color::White) {
        if can_move(game, row_st, col_st, 0, 6) {
            add_move(row_st, col_st, 0, 6, res);
        }
        if can_move(game, row_st, col_st, 0, 2) {
            add_move(row_st, col_st, 0, 2, res);
        }
    }


    if (row_st == 7) && (col_st == 4) && 
    (get_piece_color(game, get_piece_ind(game, row_st, col_st).unwrap()) == Color::Black) {
        if can_move(game, row_st, col_st, 7, 6) {
            add_move(row_st, col_st, 7, 6, res);
        }
        if can_move(game, row_st, col_st, 7, 2) {
            add_move(row_st, col_st, 7, 2, res);
        }
    }
}


pub fn add_promotion_move(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8, res : &mut Vec<String>) {
    let cell_fn = cell_to_coord(row_fn, col_fn);
    let pieces  = ["n", "b", "r", "q"];
    for piece in pieces {
        res.push(cell_to_coord(row_st, col_st) + " " + cell_fn.as_str() + " " + piece);
    }
}


pub fn add_pawn_move(color : Color, row_st : u8, col_st : u8, row_fn : u8, col_fn : u8, res : &mut Vec<String>) {
    if ((row_st != 6) && (color == Color::White)) || ((row_st != 1) && (color == Color::Black)) {
        add_move(row_st, col_st, row_fn, col_fn, res);
    } else {
        add_promotion_move(row_st, col_st, row_fn, col_fn, res);
    }
}


pub fn gen_all_moves_pawn(game : &Game, row_st : u8, col_st : u8, res : &mut Vec<String>) {

    assert!(get_piece_ind(game, row_st, col_st) != None); 

    let color = get_piece_color(game, get_piece_ind(game, row_st, col_st).unwrap());

    if color == Color::White {
        let mut row_fn;
        let mut col_fn;
        for i in 1..3 {
                row_fn = (row_st as i8) + i;
                col_fn = col_st as i8;
            if is_correct_cell_i(row_fn, col_fn) {
                if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                    add_pawn_move(Color::White, row_st, col_st, row_fn as u8, col_fn as u8, res);
                }
            }
        }
        row_fn = row_st as i8 + 1;
        col_fn = col_st as i8 + 1;
        if is_correct_cell_i(row_fn, col_fn) {
            if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                add_pawn_move(Color::White, row_st, col_st, row_fn as u8, col_fn as u8, res);
            }
        }
        row_fn = row_st as i8 + 1;
        col_fn = col_st as i8 - 1;
        if is_correct_cell_i(row_fn, col_fn) {
            if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                add_pawn_move(Color::White,row_st, col_st, row_fn as u8, col_fn as u8, res);
            }
        }
    } else {
        let mut row_fn;
        let mut col_fn;
        for i in 1..3 {
            row_fn = (row_st as i8) - i;
            col_fn = col_st as i8;
            if is_correct_cell_i(row_fn, col_fn) {
                if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                    add_pawn_move(Color::Black, row_st, col_st, row_fn as u8, col_fn as u8, res);
                }
            }
        }
        row_fn = row_st as i8 - 1;
        col_fn = col_st as i8 + 1;
        if is_correct_cell_i(row_fn, col_fn) {
            if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                add_pawn_move(Color::Black, row_st, col_st, row_fn as u8, col_fn as u8, res);
            }
        }
        row_fn = row_st as i8 - 1;
        col_fn = col_st as i8 - 1;
        if is_correct_cell_i(row_fn, col_fn) {
            if can_move(game, row_st, col_st, row_fn as u8, col_fn as u8) {
                add_pawn_move(Color::Black, row_st, col_st, row_fn as u8, row_fn as u8, res);
            }
        }
    }
    
}