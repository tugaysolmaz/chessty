// This whole module assues that provided move is valid!

use crate::engine::raw_moveset::move_vertical;
use crate::pieces::{Color, Piece, PieceType};
use crate::table::Table;

pub fn move_piece(table: &mut Table, from: &usize, to: &usize) {
    match table[to] {
        Piece {
            type_: PieceType::Empty,
            ..
        } => move_to_empty_space(table, from, to),
        _ => (), // this means a successful attack, capture peace and dump to graveyard.
    }
}

fn to_graveyard(table: &mut Table, pos: &usize) {
    table
        .graveyard
        .push(std::mem::replace(&mut table.table[*pos], Piece::empty()));
}

fn move_to_empty_space(table: &mut Table, from: &usize, to: &usize) {
    table[to] = std::mem::replace(&mut table[from], Piece::empty());
}

fn move_and_take(table: &mut Table, from: &usize, to: &usize) {
    to_graveyard(table, to);
    table[to] = std::mem::replace(&mut table.table[*from], Piece::empty());
}

fn take_en_passant(table: &mut Table, from: &usize, to: &usize) {
    // this means to position contains and psuedo pawn piece
    let pos_to_32 = *to as i32;
    match table[to] {
        Piece {
            color: Color::Black,
            ..
        } => {
            let post_to_down_32 = move_vertical(pos_to_32, false) as usize;
            to_graveyard(table, &post_to_down_32)
        }
        Piece {
            color: Color::White,
            ..
        } => {
            let post_to_up_32 = move_vertical(pos_to_32, true) as usize;
            to_graveyard(table, &post_to_up_32)
        }
        _ => panic!("Invalid en passant request!"),
    }
    move_and_take(table, from, to);
}
