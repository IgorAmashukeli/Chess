use std::cmp;

pub fn is_knight_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    let dist_col = cmp::max(col_st, col_fn) - cmp::min(col_st, col_fn);
    return ((dist_row == 1) && (dist_col == 2)) || ((dist_row == 2) && (dist_col == 1))
}

pub fn is_bishop_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    let dist_col = cmp::max(col_st, col_fn) - cmp::min(col_st, col_fn);
    return (dist_row == dist_col) && (dist_row != 0)
}

pub fn is_rook_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    let dist_col = cmp::max(col_st, col_fn) - cmp::min(col_st, col_fn);
    return ((dist_row == 0) && (dist_col != 0)) || ((dist_row != 0) && (dist_col == 0))
}

pub fn is_queen_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return is_bishop_shape(row_st, col_st, row_fn, col_fn) || is_rook_shape(row_st, col_st, row_fn, col_fn)
}

pub fn is_king_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_row = cmp::max(row_st, row_fn) - cmp::min(row_st, row_fn);
    let dist_col = cmp::max(col_st, col_fn) - cmp::min(col_st, col_fn);
    return is_queen_shape(row_st, col_st, row_fn, col_fn) && dist_row <= 1 && dist_col <= 1;
}

pub fn is_king_wh_shrt_castle_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return (row_st == row_fn) && (row_st == 0) && (col_st == 4) && (col_fn == 6);
}

pub fn is_king_wh_lng_castle_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return (row_st == row_fn) && (row_st == 0) && (col_st == 4) && (col_fn == 2);
}

pub fn is_king_blck_shrt_castle_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return (row_st == row_fn) && (row_st == 7) && (col_st == 4) && (col_fn == 6);
}

pub fn is_king_blck_lng_castle_shape(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return (row_st == row_fn) && (row_st == 7) && (col_st == 4) && (col_fn == 2);
}

pub fn is_pawn_shape_move_wht(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return (col_st == col_fn) && ((row_fn == row_st + 1) || ((row_fn == row_st + 2) && (row_st == 1)));
}

pub fn is_pawn_shape_move_blck(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return (col_st == col_fn) && ((row_st == row_fn + 1) || ((row_st == row_fn + 2) && (row_st == 6)));
}

pub fn is_pawn_shape_take_wht(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_col = cmp::max(col_fn, col_st) - cmp::min(col_fn, col_st);
    return (dist_col == 1) && (row_fn == row_st + 1)
}

pub fn is_pawn_shape_take_blck(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    let dist_col = cmp::max(col_fn, col_st) - cmp::min(col_fn, col_st);
    return (dist_col == 1) && (row_st == row_fn + 1)
}

pub fn is_pawn_shape_en_passant_wht(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return row_st == 4 && row_fn == 5 && (cmp::max(col_fn, col_st) - cmp::min(col_fn, col_st) == 1);
}

pub fn is_pawn_shape_en_passant_blck(row_st : u8, col_st : u8, row_fn : u8, col_fn : u8) -> bool {
    return row_st == 3 && row_fn == 2 && (cmp::max(col_fn, col_st) - cmp::min(col_fn, col_st) == 1);
}


