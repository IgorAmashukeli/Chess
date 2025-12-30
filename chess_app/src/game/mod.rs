use crate::cell::Cell;
use crate::game::cli::{ ACCEPT_DRAW, OFFER_DRAW, RESIGN, get_move, position_to_prefen, print_current_state, print_moves, process_offer_draw};
use crate::game::moves::{add_offers, can_move, gen_all_moves, is_under_check, make_move};
use crate::piece::{Color, PieceType, Piece, };
use std::collections::HashMap;

use std::fmt;
pub mod moves;
pub mod shapes;
pub mod reachable;
pub mod free;
pub mod generate;
pub mod cli;

pub type Board = [[Cell; 8]; 8];
pub type PieceSet = [Piece; 32];


#[derive(Debug, Clone, Copy, PartialEq)]

pub enum GameWinner {
    White,
    Black,
    Draw,
}

impl fmt::Display for GameWinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use write! to format the output into the formatter f
        match *self {
            GameWinner::White => write!(f, "{}", "White Won"),
            GameWinner::Black => write!(f, "{}", "Black Won"),
            GameWinner::Draw =>  write!(f, "{}", "Draw"),
        }
        
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameResult {
    CheckMate,
    StaleMate,
    Rule50Draw,
    TreefoldRepDraw,
    Resignation,
    AgreedDraw
}


impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use write! to format the output into the formatter f
        match *self {
            GameResult::CheckMate => write!(f, "{}", "CheckMate"),
            GameResult::StaleMate => write!(f, "{}", "StaleMate"),
            GameResult::Rule50Draw =>  write!(f, "{}", "50 moves without pawn and take moves"),
            GameResult::TreefoldRepDraw => write!(f, "{}", "Threefold repetition of position"),
            GameResult::Resignation => write!(f, "{}", "Resignation"),
            GameResult::AgreedDraw =>  write!(f, "{}", "Draw is Agreed"),
        }
        
    }
}



#[derive(Debug)]
pub struct Game {
    board : Board,
    piece_set : PieceSet,
    active_color : Color,
    white_short_castle : bool,
    white_long_castle : bool,
    black_short_castle : bool,
    black_long_castle : bool,
    enpassant_sq : Option<(u8, u8)>,
    rule50_clock : u32,
    move_number : u32,
    prefen_map : HashMap<String, u32>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board : [
            [Cell::new(Some(12)), Cell::new(Some(8)), Cell::new(Some(10)), Cell::new(Some(14)),
            Cell::new(Some(15)), Cell::new(Some(11)), Cell::new(Some(9)), Cell::new(Some(13))],
            [Cell::new(Some(0)), Cell::new(Some(1)), Cell::new(Some(2)), Cell::new(Some(3)),
            Cell::new(Some(4)), Cell::new(Some(5)), Cell::new(Some(6)), Cell::new(Some(7))],
            [Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None), 
            Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None)], 
            [Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None), 
            Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None)], 
            [Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None), 
            Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None)], 
            [Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None), 
            Cell::new(None), Cell::new(None), Cell::new(None), Cell::new(None)], 
            [Cell::new(Some(16)), Cell::new(Some(17)), Cell::new(Some(18)), Cell::new(Some(19)),
            Cell::new(Some(20)), Cell::new(Some(21)), Cell::new(Some(22)), Cell::new(Some(23))],
            [Cell::new(Some(28)), Cell::new(Some(24)), Cell::new(Some(26)), Cell::new(Some(30)),
            Cell::new(Some(31)), Cell::new(Some(27)), Cell::new(Some(25)), Cell::new(Some(29))], 
             ],
            piece_set : [
            Piece::new(Color::White, PieceType::Pawn, Some((1, 0))),
            Piece::new(Color::White, PieceType::Pawn, Some((1, 1))),
            Piece::new(Color::White, PieceType::Pawn, Some((1, 2))),
            Piece::new(Color::White, PieceType::Pawn, Some((1, 3))),
            Piece::new(Color::White, PieceType::Pawn, Some((1, 4))),
            Piece::new(Color::White, PieceType::Pawn, Some((1, 5))),
            Piece::new(Color::White, PieceType::Pawn, Some((1, 6))),
            Piece::new(Color::White, PieceType::Pawn, Some((1, 7))),

            Piece::new(Color::White, PieceType::Knight, Some((0, 1))),
            Piece::new(Color::White, PieceType::Knight, Some((0, 6))),
            Piece::new(Color::White, PieceType::Bishop, Some((0, 2))),
            Piece::new(Color::White, PieceType::Bishop, Some((0, 5))),
            Piece::new(Color::White, PieceType::Rook, Some((0, 0))),
            Piece::new(Color::White, PieceType::Rook, Some((0, 7))),
            Piece::new(Color::White, PieceType::Queen, Some((0, 3))),
            Piece::new(Color::White, PieceType::King, Some((0, 4))),

            Piece::new(Color::Black, PieceType::Pawn, Some((6, 0))),
            Piece::new(Color::Black, PieceType::Pawn, Some((6, 1))),
            Piece::new(Color::Black, PieceType::Pawn, Some((6, 2))),
            Piece::new(Color::Black, PieceType::Pawn, Some((6, 3))),
            Piece::new(Color::Black, PieceType::Pawn, Some((6, 4))),
            Piece::new(Color::Black, PieceType::Pawn, Some((6, 5))),
            Piece::new(Color::Black, PieceType::Pawn, Some((6, 6))),
            Piece::new(Color::Black, PieceType::Pawn, Some((6, 7))),

            Piece::new(Color::Black, PieceType::Knight, Some((7, 1))),
            Piece::new(Color::Black, PieceType::Knight, Some((7, 6))),
            Piece::new(Color::Black, PieceType::Bishop, Some((7, 2))),
            Piece::new(Color::Black, PieceType::Bishop, Some((7, 5))),
            Piece::new(Color::Black, PieceType::Rook, Some((7, 0))),
            Piece::new(Color::Black, PieceType::Rook, Some((7, 7))),
            Piece::new(Color::Black, PieceType::Queen, Some((7, 3))),
            Piece::new(Color::Black, PieceType::King, Some((7, 4))),
            ],
            active_color : Color::White,
            white_short_castle : true,
            white_long_castle : true,
            black_short_castle : true,
            black_long_castle : true,
            enpassant_sq : None,
            rule50_clock : 0,
            move_number : 1,
            prefen_map : HashMap::new()
        }
    }
}

impl Game {
    pub fn new(board : [[Cell; 8]; 8], piece_set : [Piece; 32], 
        active_color : Color, 
        white_short_castle : bool, white_long_castle : bool,
        black_short_castle : bool, black_long_castle : bool,
        enpassant_sq : Option<(u8, u8)>, rule50_clock : u32,
        move_number : u32,
        prefen_map : HashMap<String, u32>
    ) -> Game {
        Game{board, piece_set,
            active_color,
            white_short_castle, white_long_castle,
            black_short_castle, black_long_castle,
            enpassant_sq, rule50_clock,
            move_number,
            prefen_map
        }
    }

    pub fn play_cli(&mut self) -> (GameResult, GameWinner, u32) {

        let mut was_draw_offer = false;
        
        while self.rule50_clock <= 100 {

            // get all moves
            let mut moves = gen_all_moves(self);

            // print board
            print_current_state(self);


            // if no moves => checkmate/stalemate
            if moves.is_empty() {
                if is_under_check(self) {
                    if self.active_color == Color::White {
                        return (GameResult::CheckMate, GameWinner::Black, self.move_number);
                    } else {
                        return (GameResult::CheckMate, GameWinner::White, self.move_number);
                    }
                } else {
                    return (GameResult::StaleMate, GameWinner::Draw, self.move_number);
                }
            }

            // if rule 50 works, draw
            if self.rule50_clock == 100 {
                return (GameResult::Rule50Draw, GameWinner::Draw, self.move_number);
            }

            // increase current position count
            // if it already has at least 2 repetitons, draw
            let prefen = position_to_prefen(self);
            if let Some(count) = self.prefen_map.get_mut(&prefen) {
                if *count >= 2 {
                    return (GameResult::TreefoldRepDraw, GameWinner::Draw, self.move_number);
                } else {
                    *count += 1;
                }
            } else {
                self.prefen_map.insert(prefen, 1);
            }

            // add draw and resign offers
            add_offers(&mut moves, was_draw_offer);

            // print all possible moves
            print_moves(&moves);

            // get the move from cli
            let (mut row_st, mut col_st, mut row_fn, mut col_fn, mut promotion_type) = get_move(&moves);

            // process offer/accept/resign
            if row_st == OFFER_DRAW {

                process_offer_draw(&mut was_draw_offer);
                moves = gen_all_moves(self);
                print_moves(&moves);
                (row_st, col_st, row_fn, col_fn, promotion_type) = get_move(&moves);

            } else if row_st == ACCEPT_DRAW {
                return (GameResult::AgreedDraw, GameWinner::Draw, self.move_number);
            } else if (row_st == RESIGN) && (self.active_color == Color::White) {
                return (GameResult::Resignation, GameWinner::Black, self.move_number);
            } else if (row_st == RESIGN) && (self.active_color == Color::Black) {
                return (GameResult::Resignation, GameWinner::White, self.move_number);
            } else {
                was_draw_offer = false;
            }


            // assert it is a correct move (due to the fact it was taken directly from the list of all possible)
            assert!(can_move(self, row_st, col_st, row_fn, col_fn));
            
            // move
            make_move(self, row_st, col_st, row_fn, col_fn, promotion_type);



        }

        // should not be reachable
        return (GameResult::Rule50Draw, GameWinner::Draw, self.move_number);
    }

}