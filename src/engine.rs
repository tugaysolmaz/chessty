use crate::pieces::{Color, Piece};
use crate::table::Table;

pub fn possible_moves(table: Table, pos: usize) -> Vec<usize> {
    let possible_moves: Vec<usize> = vec![];
    match table.table[pos] {
        Piece::Bishop(Color::Black) => {}
        Piece::Bishop(Color::White) => (),
        Piece::King(Color::Black) => (),
        Piece::King(Color::White) => (),
        Piece::Knight(Color::Black) => (),
        Piece::Knight(Color::White) => (),
        Piece::Pawn(Color::Black) => (),
        Piece::Pawn(Color::White) => (),
        Piece::Queen(Color::Black) => (),
        Piece::Queen(Color::White) => (),
        Piece::Rook(Color::Black) => (),
        Piece::Rook(Color::White) => (),
        _ => (),
    };
    possible_moves
}

pub fn move_diagonally(pos: isize, up: bool, right: bool) -> isize {
    move_horizontal(move_vertical(pos, up), right)
}

pub fn move_vertical(pos: isize, up: bool) -> isize {
    if !valid_pos(pos) {
        return pos;
    }

    if up && pos + 8 < 64 {
        pos + 8
    } else if !up && pos - 8 > -1 {
        pos - 8
    } else {
        -1
    }
}

pub fn move_horizontal(pos: isize, right: bool) -> isize {
    if !valid_pos(pos) {
        return pos;
    }
    if right && (pos % 8 + 1 < 8) {
        pos + 1
    } else if !right && (pos % 8 - 1 > -1) {
        pos - 1
    } else {
        -1
    }
}

pub fn valid_pos(pos: isize) -> bool {
    pos < 64 && pos > -1
}

pub fn knight_moves(pos: isize) -> Vec<isize> {
    vec![
        move_vertical(move_vertical(move_horizontal(pos, true), true), true),
        move_vertical(move_vertical(move_horizontal(pos, false), true), true),
        move_vertical(move_vertical(move_horizontal(pos, true), false), false),
        move_vertical(move_vertical(move_horizontal(pos, false), false), false),
        move_vertical(move_horizontal(move_horizontal(pos, true), true), true),
        move_vertical(move_horizontal(move_horizontal(pos, true), true), false),
        move_vertical(move_horizontal(move_horizontal(pos, false), false), true),
        move_vertical(move_horizontal(move_horizontal(pos, false), false), true),
    ]
    .into_iter()
    .filter(|&pos| pos != -1)
    .collect()
}

pub fn bishop_moves(pos: isize) -> Vec<isize> {
    let mut res = vec![];

    let directions = [[true, true], [true, false], [false, false], [false, true]];
    for dpair in directions {
        loop {
            let new_pos = move_horizontal(move_vertical(pos, dpair[1]), dpair[0]);
            if new_pos == -1 {
                break;
            } else {
                res.push(new_pos);
            }
        }
    }

    res
}

pub fn check_legality() {}

pub fn pos_int2str() {}
pub fn pos_str2int() {}
