#[derive(Clone)]
pub enum Color {
    White,
    Black,
    None,
}

#[derive(Clone)]
pub enum Piece {
    Empty, // color
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}
