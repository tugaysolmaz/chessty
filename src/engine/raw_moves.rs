use crate::pieces::{Color, Piece};
use crate::table::{Table, ALPHABET, STANDARD_SIZE};

pub fn get_sqrt_table_size() -> i32 {
    (STANDARD_SIZE as f64).sqrt() as i32
}

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

pub fn move_diagonally(pos: i32, up: bool, right: bool) -> i32 {
    move_horizontal(move_vertical(pos, up), right)
}

pub fn move_vertical(pos: i32, up: bool) -> i32 {
    if !valid_pos(&pos) {
        return pos;
    }

    let sqrt = get_sqrt_table_size();

    if up && pos + sqrt < STANDARD_SIZE {
        pos + 8
    } else if !up && pos - sqrt > -1 {
        pos - 8
    } else {
        -1
    }
}

pub fn move_horizontal(pos: i32, right: bool) -> i32 {
    if !valid_pos(&pos) {
        return pos;
    }

    let sqrt = get_sqrt_table_size();

    if right && (pos % sqrt + 1 < sqrt) {
        pos + 1
    } else if !right && (pos % sqrt - 1 > -1) {
        pos - 1
    } else {
        -1
    }
}

pub fn valid_pos(pos: &i32) -> bool {
    *pos < STANDARD_SIZE && *pos > -1
}

pub fn apppend_valid_pos(pos_list: &mut Vec<i32>, pos: &i32) -> bool {
    if valid_pos(pos) && !pos_list.contains(pos) {
        pos_list.push(pos.clone());
        return true;
    }
    false
}

pub fn pos_int2str(pos: &i32) -> String {
    let sqrt = get_sqrt_table_size();
    let upos = *pos as usize;
    format!(
        "{}{}",
        ALPHABET.chars().nth(upos % (sqrt as usize)).unwrap(),
        pos / 8 + 1
    )
}

pub fn pos_str2int(pos: &str) -> i32 {
    let sqrt = get_sqrt_table_size();
    let chars = pos.chars();
    let remainder = ALPHABET
        .chars()
        .position(|c| c == *&chars.clone().nth(0).unwrap())
        .unwrap() as i32;
    let multiplier = chars.clone().nth(1).unwrap().to_digit(10).unwrap() as i32 - 1;
    multiplier * sqrt + remainder
}

pub fn knight_moves(pos: i32) -> Vec<i32> {
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

pub fn bishop_moves(pos: i32) -> Vec<i32> {
    let mut res = vec![];

    let directions = [[true, true], [true, false], [false, false], [false, true]];
    for dpair in directions {
        let mut new_pos = pos;
        loop {
            new_pos = move_horizontal(move_vertical(new_pos, dpair[1]), dpair[0]);
            let _ = apppend_valid_pos(&mut res, &new_pos);
        }
    }

    return res;
}

pub fn rook_moves(pos: i32) -> Vec<i32> {
    let mut res = vec![];

    let directions = [true, false];
    for d in directions {
        let mut new_pos = pos;
        loop {
            new_pos = move_vertical(new_pos, d);
            if !apppend_valid_pos(&mut res, &new_pos) {
                break;
            }
        }
        loop {
            new_pos = move_horizontal(new_pos, d);
            if !apppend_valid_pos(&mut res, &new_pos) {
                break;
            }
        }
    }
    return res;
}

pub fn queen_moves(pos: i32) -> Vec<i32> {
    let mut res = rook_moves(pos.clone());
    res.append(&mut bishop_moves(pos));
    res
}

pub fn king_moves(pos: i32) -> Vec<i32> {
    let size = get_sqrt_table_size();
    let mut res = vec![];
    apppend_valid_pos(&mut res, &(pos + 1));
    apppend_valid_pos(&mut res, &(pos + size + 1));
    apppend_valid_pos(&mut res, &(pos + size));
    apppend_valid_pos(&mut res, &(pos + size - 1));
    apppend_valid_pos(&mut res, &(pos - 1));
    apppend_valid_pos(&mut res, &(pos - size - 1));
    apppend_valid_pos(&mut res, &(pos - size));
    apppend_valid_pos(&mut res, &(pos - size + 1));
    res
}

pub fn pawn_moves(pos: i32, up: bool) -> Vec<i32> {
    let size = get_sqrt_table_size();
    if up {
        return vec![pos + size];
    } else {
        return vec![pos - size];
    }
}
