use crate::table::{ALPHABET, SIZE_SQUARE, STANDARD_SIZE};

pub fn valid_pos(pos: &i32) -> bool {
    pos < &STANDARD_SIZE && pos > &-1
}

pub fn apppend_valid_pos(pos_list: &mut Vec<i32>, pos: &i32) -> bool {
    if valid_pos(pos) && !pos_list.contains(pos) {
        pos_list.push(pos.clone());
        return true;
    }
    false
}

pub fn move_diagonally(pos: i32, up: bool, right: bool) -> i32 {
    move_horizontal(move_vertical(pos, up), right)
}

pub fn move_vertical(pos: i32, up: bool) -> i32 {
    if !valid_pos(&pos) {
        return pos;
    }

    if up && pos + SIZE_SQUARE < STANDARD_SIZE {
        pos + 8
    } else if !up && pos - SIZE_SQUARE > -1 {
        pos - 8
    } else {
        -1
    }
}

pub fn move_horizontal(pos: i32, right: bool) -> i32 {
    if !valid_pos(&pos) {
        return pos;
    }

    if right && (pos % SIZE_SQUARE + 1 < SIZE_SQUARE) {
        pos + 1
    } else if !right && (pos % SIZE_SQUARE - 1 > -1) {
        pos - 1
    } else {
        -1
    }
}

pub fn pos_arr2int(pos: [i32; 2]) -> i32 {
    pos[0] + pos[1] * SIZE_SQUARE
}

pub fn pos_int2str(pos: &i32) -> String {
    let upos = *pos as usize;
    format!(
        "{}{}",
        ALPHABET.chars().nth(upos % (SIZE_SQUARE as usize)).unwrap(),
        pos / 8 + 1
    )
}

pub fn pos_str2int(pos: &str) -> i32 {
    let chars = pos.chars();
    let remainder = ALPHABET
        .chars()
        .position(|c| c == *&chars.clone().nth(0).unwrap())
        .unwrap() as i32;
    let multiplier = chars.clone().nth(1).unwrap().to_digit(10).unwrap() as i32 - 1;
    multiplier * SIZE_SQUARE + remainder
}
