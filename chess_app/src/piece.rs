#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub color : Color,
    pub piece_type : PieceType,
    pub cell : Option<(u8, u8)>,
}

impl Piece {
    pub fn new(color : Color, piece_type : PieceType, cell : Option<(u8, u8)>) -> Piece {
        Piece {color, piece_type, cell}
    }
}
