use cheest::engine::raw_moves;

#[test]
fn test_move_diagonally() {
    assert_eq!(9, raw_moves::move_diagonally(0, true, true)); // move up right

    assert_eq!(8, raw_moves::move_diagonally(1, true, false)); // move up left

    assert_eq!(1, raw_moves::move_diagonally(10, false, false)); // move down left

    assert_eq!(3, raw_moves::move_diagonally(10, false, true)); // move down right

    assert_eq!(-1, raw_moves::move_diagonally(0, false, false)); // invalid move, move from 0 to down left
    assert_eq!(-1, raw_moves::move_diagonally(0, false, true)); // invalid move, move from 0 to down left

    assert_eq!(-1, raw_moves::move_diagonally(62, true, false)); // invalid move, move from b8 to down left
    assert_eq!(-1, raw_moves::move_diagonally(62, true, true)); // invalid move, move from b8 to down left
}

#[test]
fn test_move_vertical() {
    assert_eq!(8, raw_moves::move_vertical(0, true));
    assert_eq!(48, raw_moves::move_vertical(56, false));

    assert_eq!(-1, raw_moves::move_vertical(56, true));
    assert_eq!(-1, raw_moves::move_vertical(0, false));
}

#[test]
fn test_move_horizontal() {
    assert_eq!(1, raw_moves::move_horizontal(0, true));
    assert_eq!(61, raw_moves::move_horizontal(62, false));

    assert_eq!(-1, raw_moves::move_horizontal(63, true));
    assert_eq!(-1, raw_moves::move_horizontal(0, false));
}

#[test]
fn test_pos_int2str() {
    assert_eq!("a1", raw_moves::pos_int2str(&0));
    assert_eq!("h1", raw_moves::pos_int2str(&7));

    assert_eq!("a8", raw_moves::pos_int2str(&56));
    assert_eq!("b8", raw_moves::pos_int2str(&57));

    assert_eq!("h8", raw_moves::pos_int2str(&63));
    assert_eq!("g8", raw_moves::pos_int2str(&62));
}

#[test]
fn test_pos_str2int() {
    assert_eq!(0, raw_moves::pos_str2int(&"a1"));
    assert_eq!(7, raw_moves::pos_str2int(&"h1"));

    assert_eq!(56, raw_moves::pos_str2int(&"a8"));
    assert_eq!(57, raw_moves::pos_str2int(&"b8"));

    assert_eq!(63, raw_moves::pos_str2int(&"h8"));
    assert_eq!(62, raw_moves::pos_str2int(&"g8"));
}

fn test_bishop_moves() {}
fn test_rook_moves() {}
