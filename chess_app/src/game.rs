use crate::cell::*;
use crate::piece::*;

#[derive(Debug)]
pub struct Game {
    board : [[Cell; 8]; 8],
    piece_set : [Piece; 32],
    active_color : Color,
    white_short_castle : bool,
    white_long_castle : bool,
    black_short_castle : bool,
    black_long_castle : bool,
    enpassant_sq : Option<(u8, u8)>,
    rule50_clock : u32,
    move_number : u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board : [
            [Cell::new(Some(12)), Cell::new(Some(8)), Cell::new(Some(10)), Cell::new(Some(14)),
            Cell::new(Some(15)), Cell::new(Some(11)), Cell::new(Some(19)), Cell::new(Some(13))],
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
            piece_set : [Piece::new(Color::White, PieceType::Pawn, Some((1, 0))),
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
            move_number : 1
        }
    }
}

impl Game {
    pub fn new(board : [[Cell; 8]; 8], piece_set : [Piece; 32], 
        active_color : Color, 
        white_short_castle : bool, white_long_castle : bool,
        black_short_castle : bool, black_long_castle : bool,
        enpassant_sq : Option<(u8, u8)>, rule50_clock : u32,
        move_number : u32
    ) -> Game {
        Game{board, piece_set,
            active_color,
            white_short_castle, white_long_castle,
            black_short_castle, black_long_castle,
            enpassant_sq, rule50_clock,
            move_number
        }
    }

    pub fn play() {
        
    }
}

