use crate::pieces::Color;
use crate::pieces::Piece;

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
            table: vec![Piece::Empty; 64],
            graveyard: vec![Piece::Empty; 0],
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
                0 => Piece::Rook(Color::White), // this is a1 position
                1 => Piece::Knight(Color::White),
                2 => Piece::Bishop(Color::White),
                3 => Piece::Queen(Color::White),
                4 => Piece::King(Color::White),
                5 => Piece::Bishop(Color::White),
                6 => Piece::Knight(Color::White),
                7 => Piece::Rook(Color::White),
                8..=15 => Piece::Pawn(Color::White),
                47..=55 => Piece::Pawn(Color::Black),
                56 => Piece::Rook(Color::Black),
                57 => Piece::Knight(Color::Black),
                58 => Piece::Bishop(Color::Black),
                59 => Piece::Queen(Color::Black),
                60 => Piece::King(Color::Black),
                61 => Piece::Bishop(Color::Black),
                62 => Piece::Knight(Color::Black),
                63 => Piece::Rook(Color::Black),
                _ => Piece::Empty,
            };
        }
    }

    fn increment_turn(&mut self) {
        self.n_turn += 1;
    }

    pub fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Color::Black => Color::White,
            Color::White => Color::Black,
            _ => Color::White,
        }
    }

    pub fn move_piece(&mut self, from: i32, to: i32) -> bool {
        if self.is_possible(from, to) {
            match self.table[to as usize] {
                Piece::Empty => (),
                _ => {
                    self.graveyard.push(self.table[to as usize].clone());
                }
            };
            self.table[to as usize] = self.table[from as usize].clone();
            self.table[from as usize] = Piece::Empty;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn is_possible(&self, _from: i32, _to: i32) -> bool {
        true
    }

    #[allow(dead_code)]
    pub fn get_possible_moves(&self, _where_: i32) -> Vec<i32> {
        vec![0]
    }
}
