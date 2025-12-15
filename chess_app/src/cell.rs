#[derive(Copy, Clone, Debug)]
pub struct Cell {
    piece : Option<u8>
}

impl Cell {
    pub fn new(piece : Option<u8>) -> Cell {
        Cell {piece}
    }
    pub fn is_free(&self) -> bool {
        self.piece == None
    }
}