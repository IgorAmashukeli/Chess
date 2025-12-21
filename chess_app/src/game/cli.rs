use crate::{game::{Game, moves::*, reachable::*}, piece::*};
use std::io::{self, BufRead};

pub fn cell_to_str(game : &Game, row : u8, col : u8) -> String {
    let piece_ind_opt = get_piece_ind(game, row, col);
    if piece_ind_opt == None {
        return ".".to_string();
    }

    let piece_ind = piece_ind_opt.unwrap();

    let color = get_piece_color(game, piece_ind);

    let piece_type = get_piece_type(game, piece_ind);

    match piece_type {
        PieceType::Knight => if color == Color::White {"N".to_string()} else {"n".to_string()},
        PieceType::Bishop => if color == Color::White {"B".to_string()} else {"b".to_string()},
        PieceType::Rook => if color == Color::White {"R".to_string()} else {"r".to_string()},
        PieceType::Queen => if color == Color::White {"Q".to_string()} else {"q".to_string()},
        PieceType::King => if color == Color::White {"K".to_string()} else {"k".to_string()},
        PieceType::Pawn => if color == Color::White {"P".to_string()} else {"p".to_string()},
    }
}


pub fn game_to_str(game : &Game) -> String {
    let mut result = "".to_string();
    for i in 0..8 {
        for j in 0..8 {
            result += cell_to_str(game, 7 - i, j).as_str();
            result += " ";
        }
        result += "\n";
    }

    return result;
}

pub fn print_moves(game : &Game) {
    println!("Possible moves:\n");
    let res = gen_all_moves(game);
    let mut count = 1;
    for elem in res {
        println!("{}) {}", count, elem);
        count += 1;
    }
}


pub fn print_current_state(game : &Game) {
    println!("{}", game_to_str(game));
    print_moves(game);
}


pub fn coord_to_cell(coord : &str) -> Option<(u8, u8)> {
    if coord.len() != 2 {
        return None;
    }

    let bytes = coord.as_bytes();
    if bytes[0] < b'a' {
        return None;
    }
    if bytes[1] < b'1' {
        return None;
    }
    let col = bytes[0] - b'a';
    let row = bytes[1] - b'1';

    if !is_correct_cell(row, col) {
        return None
    }

    return Some((row, col));
    
}

pub fn piece_type_to_cell(piece_type_str : &str) -> Option<PieceType> {
    if piece_type_str == "n" {
        return Some(PieceType::Knight);
    } else if piece_type_str == "b" {
        return Some(PieceType::Bishop);
    } else if piece_type_str == "r" {
        return Some(PieceType::Rook);
    } else if piece_type_str == "q" {
        return Some(PieceType::Queen);
    } else {
        return None;
    }
}


pub fn move_to_values(move_str : &str) -> Option<(u8, u8, u8, u8, Option<PieceType>)> {
    let parts : Vec<&str> = move_str.split(" ").collect();
    if (parts.len() != 2) && (parts.len() != 3) {
        return None;
    }

    
    if let Some(cell_st) = coord_to_cell(parts[0]) {
        if let Some(cell_fn) = coord_to_cell(parts[1]) {
            if parts.len() == 2 {
                return Some((cell_st.0, cell_st.1, cell_fn.0, cell_fn.1, None));
            }

            if let Some(piece_type) = piece_type_to_cell(parts[2]) {
                return Some((cell_st.0, cell_st.1, cell_fn.0, cell_fn.1, Some(piece_type)))
            }

            return None;
            
        }

        return None;
    }

    return None;
    
}


pub fn read_u32() -> Option<u32> {
    let mut line = String::new();
    if io::stdin().lock().read_line(&mut line).is_ok() {
        line.trim().parse::<u32>().ok()
    } else {
        None
    }
}

pub fn read_u32_until_valid(val : &mut Option<u32>, limit : u32) {
    println!("\nSelect the number of move from the list");
    while val.is_none() {
        *val = read_u32();
        if val.is_none() {
            println!("Wrong input, not a number, try again");
            continue;
        }
        if val.unwrap() > limit {
            println!("Wrong input, too big number, try again");
            continue;
        }
    }
}


pub fn position_to_prefen(game : &Game) -> String {
    let mut result = "".to_string();
    for i in 0..8 {
        let mut count = 0;
        for j in 0..8 {
            let cell_str = cell_to_str(game, 7 - i, j);
            if cell_str == "." {
                count += 1;
            } else if count != 0 {
                result += count.to_string().as_str();
                result += cell_str.as_str();
                count = 0;
            } else {
                result += cell_str.as_str();
            }
        }

        if count != 0 {
            result += count.to_string().as_str();
        }

        if i != 7 {
            result += "/";
        }
    }

    result += " ";
    result += if game.active_color == Color::White {"w"} else {"b"};
    result += " ";
    
    if !game.white_short_castle && !game.white_long_castle && !game.black_short_castle && !game.black_long_castle {
        result += "-";
    } else {
        if game.white_short_castle {
            result += "K";
        }
        if game.white_long_castle {
            result += "Q";
        }
        if game.black_short_castle {
            result += "k";
        }
        if game.black_long_castle {
            result += "q";
        }
    }

    result += " ";

    if let Some(cell) = game.enpassant_sq {
        let coord = cell_to_coord(cell.0, cell.1);
        result += coord.as_str();
    } else {
        result += "-";
    }

    return result;
}



pub fn get_move(moves : &Vec<String>) -> (u8, u8, u8, u8, Option<PieceType>) {
    let mut val: Option<u32> = None;
    read_u32_until_valid(&mut val, moves.len() as u32);
    
    let ind = val.unwrap();
    assert! (ind >= 1);
    
    let piece_move_str = moves[ind as usize - 1].as_str();
    let cur_move_opt = move_to_values(piece_move_str);

    assert!(cur_move_opt != None);
    return cur_move_opt.unwrap();
}