use crate::{game::{Game, moves::*, reachable::*}, piece::{self, *}};

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
    for elem in res {
        println!("{}", elem);
    }
}


pub fn print_current_state(game : &Game) {
    println!("{}", game_to_str(game));
    print_moves(game);
}