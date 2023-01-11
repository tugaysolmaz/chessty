use crate::{
    pieces::PieceType,
    table::{Table, SIZE_SQUARE},
};

use super::{
    raw_moveset::{apppend_valid_pos, move_horizontal, move_vertical, valid_pos},
    validator::{filter_moveset, validate_move},
};

fn push_valid(moveset: &mut Vec<i32>, valid: bool, new_pos: &i32) -> bool {
    if valid {
        moveset.push(*new_pos);
    }
    valid
}

pub fn knight_moves(table: &mut Table, pos: i32) -> Vec<i32> {
    let mut moveset = vec![
        move_vertical(move_vertical(move_horizontal(pos, true), true), true),
        move_vertical(move_vertical(move_horizontal(pos, false), true), true),
        move_vertical(move_vertical(move_horizontal(pos, true), false), false),
        move_vertical(move_vertical(move_horizontal(pos, false), false), false),
        move_horizontal(move_horizontal(move_vertical(pos, true), true), true),
        move_horizontal(move_horizontal(move_vertical(pos, false), true), true),
        move_horizontal(move_horizontal(move_vertical(pos, true), false), false),
        move_horizontal(move_horizontal(move_vertical(pos, false), false), false),
    ]
    .into_iter()
    .filter(|&pos| pos != -1)
    .collect();

    filter_moveset(table, &pos, moveset)
}

pub fn bishop_moves(table: &mut Table, pos: i32) -> Vec<i32> {
    let mut moveset = vec![];
    let mut valid: bool;

    let directions = [[true, true], [true, false], [false, false], [false, true]];
    for dpair in directions {
        let mut new_pos = pos;
        loop {
            new_pos = move_horizontal(move_vertical(new_pos, dpair[1]), dpair[0]);
            if !push_valid(&mut moveset, validate_move(table, &pos, &new_pos), &new_pos) {
                break;
            }
        }
    }

    return moveset;
}

pub fn rook_moves(table: &mut Table, pos: i32) -> Vec<i32> {
    let mut moveset = vec![];

    let directions = [true, false];
    for di in directions {
        let mut new_pos = pos.clone();
        loop {
            new_pos = move_vertical(new_pos, di);
            if !push_valid(&mut moveset, validate_move(table, &pos, &new_pos), &new_pos) {
                break;
            }
        }
        new_pos = pos.clone();
        loop {
            new_pos = move_horizontal(new_pos, di);
            if !push_valid(&mut moveset, validate_move(table, &pos, &new_pos), &new_pos) {
                break;
            }
        }
    }
    return moveset;
}

pub fn queen_moves(table: &mut Table, pos: i32) -> Vec<i32> {
    let mut res = rook_moves(table, pos.clone());
    res.append(&mut bishop_moves(table, pos));
    res
}

pub fn king_moves(table: &mut Table, pos: i32) -> Vec<i32> {
    // TODO: CASTLING IS MISSING
    let mut moveset = vec![
        move_vertical(move_horizontal(pos, false), true),
        move_vertical(pos, true),
        move_vertical(move_horizontal(pos, true), true),
        move_horizontal(pos, false),
        move_horizontal(pos, true),
        move_vertical(move_horizontal(pos, false), false),
        move_vertical(pos, false),
        move_vertical(move_horizontal(pos, true), false),
    ];
    moveset = moveset.into_iter().filter(|x| valid_pos(x)).collect();
    filter_moveset(table, &pos, moveset)
}

pub fn pawn_moves(table: &mut Table, pos: i32, up: bool) -> Vec<i32> {
    // TODO: en passant, taking
    let mut moveset: Vec<i32> = vec![];
    let new_direction = if up {
        pos + SIZE_SQUARE
    } else {
        pos - SIZE_SQUARE
    };
    if valid_pos(&new_direction) {
        if table[&new_direction].type_ == PieceType::Empty {
            moveset.push(new_direction);
        }
    }

    let new_left = new_direction - 1;
    if valid_pos(&new_left) {
        if table[&new_left].color != table[&pos].color {
            moveset.push(new_left);
        }
    }

    let new_right = new_direction + 1;
    if valid_pos(&new_right) {
        if table[&new_right].color != table[&pos].color {
            moveset.push(new_right);
        }
    }

    filter_moveset(table, &pos, moveset)
}
