#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
    None,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            _ => Color::None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum PieceType {
    Empty, // color
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone)]
pub struct Piece {
    pub type_: PieceType,
    pub pos: i32,
    pub color: Color,
    pub en_passantable: bool,
}

impl Piece {
    pub fn new(pos: i32, type_: PieceType, color: Color) -> Self {
        Piece {
            type_: type_,
            pos: pos,
            color: color,
            en_passantable: false,
        }
    }

    pub fn empty() -> Self {
        Piece {
            type_: PieceType::Empty,
            pos: -1,
            color: Color::None,
            en_passantable: false,
        }
    }
}
