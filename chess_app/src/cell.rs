use crate::{piece::Color};

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub piece : Option<u8>,
}

impl Cell {
    pub fn new(piece : Option<u8>) -> Cell {
        Cell {piece}
    }

    pub fn is_free(&self) -> bool {
        self.piece == None
    }
}
