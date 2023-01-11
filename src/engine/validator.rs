use crate::engine::actions::move_piece;
use crate::{
    pieces::{Piece, PieceType},
    table::Table,
};

use super::raw_moveset::valid_pos;

pub fn filter_moveset(table: &mut Table, from: &i32, moveset: Vec<i32>) -> Vec<i32> {
    moveset
        .into_iter()
        .filter(|to| validate_move(table, from, to))
        .collect()
}

pub fn validate_move(table: &mut Table, from: &i32, to: &i32) -> bool {
    // we already assume from and to are in table dimensions
    if !valid_pos(&to) {
        println!("Invalid mode, not valid");
        return false;
    }
    if takeable(table, from, to) {
        println!("Invalid mode, not takeble");
        return false;
    }
    if self_checks(table, from, to) {
        println!("Invalid mode, self");
        return false;
    }
    return true;
}

#[allow(dead_code)]
fn self_checks(table: &mut Table, from: &i32, to: &i32) -> bool {
    false // TODO
}

fn takeable(table: &mut Table, from: &i32, to: &i32) -> bool {
    table[to].color != table[from].color
}
