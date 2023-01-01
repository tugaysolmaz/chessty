use crate::pieces::Color;
use crate::pieces::{Piece, PieceType};

pub const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
pub const SIZE_SQUARE: i32 = 8;
pub const STANDARD_SIZE: i32 = SIZE_SQUARE * SIZE_SQUARE;

pub struct Table {
    pub n_turn: i32,
    pub turn: Color,
    pub table: Vec<Piece>, // squared size
    pub graveyard: Vec<Piece>,
}

impl Table {
    fn new_empty() -> Table {
        Table {
            n_turn: 0,
            turn: Color::White,
            table: vec![Piece::empty(); 64],
            graveyard: vec![Piece::empty(); 0],
        }
    }

    pub fn new_standard() -> Table {
        let mut table = Table::new_empty();
        table.populate_standard();
        table
    }

    pub fn populate_standard(&mut self) {
        for num in 0..self.table.len() {
            self.table[num] = match num {
                0 => Piece::new(0, PieceType::Rook, Color::White), // this is a1 position
                1 => Piece::new(1, PieceType::Knight, Color::White),
                2 => Piece::new(2, PieceType::Bishop, Color::White),
                3 => Piece::new(3, PieceType::Queen, Color::White),
                4 => Piece::new(4, PieceType::King, Color::White),
                5 => Piece::new(5, PieceType::Bishop, Color::White),
                6 => Piece::new(6, PieceType::Knight, Color::White),
                7 => Piece::new(7, PieceType::Rook, Color::White),
                8..=15 => Piece::new(7, PieceType::Pawn, Color::White),
                47..=55 => Piece::new(56, PieceType::Pawn, Color::Black),
                56 => Piece::new(56, PieceType::Rook, Color::Black),
                57 => Piece::new(57, PieceType::Knight, Color::Black),
                58 => Piece::new(58, PieceType::Bishop, Color::Black),
                59 => Piece::new(59, PieceType::Queen, Color::Black),
                60 => Piece::new(60, PieceType::King, Color::Black),
                61 => Piece::new(61, PieceType::Bishop, Color::Black),
                62 => Piece::new(62, PieceType::Knight, Color::Black),
                63 => Piece::new(63, PieceType::Rook, Color::Black),
                _ => Piece::empty(),
            };
        }
    }
}
