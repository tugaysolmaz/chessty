#[derive(Clone)]
pub enum Color {
    White,
    Black,
    None,
}

#[derive(Clone)]
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

    pub fn is_en_passantable(&self) {
        match self.type_ {
            PieceType::Pawn => (),
            _ => (),
        }
    }
}
