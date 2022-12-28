use cheest::engine;

#[test]
fn test_move_diagonally() {
    assert_eq!(9, engine::move_diagonally(0, true, true)); // move up right

    assert_eq!(8, engine::move_diagonally(1, true, false)); // move up left

    assert_eq!(1, engine::move_diagonally(10, false, false)); // move down left

    assert_eq!(3, engine::move_diagonally(10, false, true)); // move down right

    assert_eq!(-1, engine::move_diagonally(0, false, false)); // invalid move, move from 0 to down left
    assert_eq!(-1, engine::move_diagonally(0, false, true)); // invalid move, move from 0 to down left

    assert_eq!(-1, engine::move_diagonally(62, true, false)); // invalid move, move from b8 to down left
    assert_eq!(-1, engine::move_diagonally(62, true, true)); // invalid move, move from b8 to down left
}

#[test]
fn test_move_vertical() {
    assert_eq!(8, engine::move_vertical(0, true));
    assert_eq!(48, engine::move_vertical(56, false));

    assert_eq!(-1, engine::move_vertical(56, true));
    assert_eq!(-1, engine::move_vertical(0, false));
}

#[test]
fn test_move_horizontal() {
    assert_eq!(1, engine::move_horizontal(0, true));
    assert_eq!(61, engine::move_horizontal(62, false));

    assert_eq!(-1, engine::move_horizontal(63, true));
    assert_eq!(-1, engine::move_horizontal(0, false));
}

#[test]
fn test_horse_moves() {
    assert_eq!(1, engine::move_horizontal(0, true));
}
