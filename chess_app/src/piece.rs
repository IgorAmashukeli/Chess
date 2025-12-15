#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    color : Color,
    piece_type : PieceType,
    cell : Option<(u8, u8)>
}

impl Piece {
    pub fn new(color : Color, piece_type : PieceType, cell : Option<(u8, u8)>) -> Piece {
        Piece {color, piece_type, cell}
    }

    pub fn is_on_board(&self) -> bool {
        return self.cell != None
    }
}