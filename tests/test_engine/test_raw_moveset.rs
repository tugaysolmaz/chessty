use cheest::engine::raw_moveset;
use cheest::engine::raw_moveset::pos_arr2int as a2i;

#[test]
fn test_move_diagonally() {
    assert_eq!(9, raw_moveset::move_diagonally(0, true, true)); // move up right

    assert_eq!(8, raw_moveset::move_diagonally(1, true, false)); // move up left

    assert_eq!(1, raw_moveset::move_diagonally(10, false, false)); // move down left

    assert_eq!(3, raw_moveset::move_diagonally(10, false, true)); // move down right

    assert_eq!(-1, raw_moveset::move_diagonally(0, false, false)); // invalid move, move from 0 to down left
    assert_eq!(-1, raw_moveset::move_diagonally(0, false, true)); // invalid move, move from 0 to down left

    assert_eq!(-1, raw_moveset::move_diagonally(62, true, false)); // invalid move, move from b8 to down left
    assert_eq!(-1, raw_moveset::move_diagonally(62, true, true)); // invalid move, move from b8 to down left
}

#[test]
fn test_move_vertical() {
    assert_eq!(8, raw_moveset::move_vertical(0, true));
    assert_eq!(48, raw_moveset::move_vertical(56, false));

    assert_eq!(-1, raw_moveset::move_vertical(56, true));
    assert_eq!(-1, raw_moveset::move_vertical(0, false));
}

#[test]
fn test_move_horizontal() {
    assert_eq!(1, raw_moveset::move_horizontal(0, true));
    assert_eq!(61, raw_moveset::move_horizontal(62, false));

    assert_eq!(-1, raw_moveset::move_horizontal(63, true));
    assert_eq!(-1, raw_moveset::move_horizontal(0, false));
}

#[test]
fn test_pos_int2str() {
    assert_eq!("a1", raw_moveset::pos_int2str(&0));
    assert_eq!("h1", raw_moveset::pos_int2str(&7));

    assert_eq!("a8", raw_moveset::pos_int2str(&56));
    assert_eq!("b8", raw_moveset::pos_int2str(&57));

    assert_eq!("h8", raw_moveset::pos_int2str(&63));
    assert_eq!("g8", raw_moveset::pos_int2str(&62));
}

#[test]
fn test_pos_str2int() {
    assert_eq!(0, raw_moveset::pos_str2int(&"a1"));
    assert_eq!(7, raw_moveset::pos_str2int(&"h1"));

    assert_eq!(56, raw_moveset::pos_str2int(&"a8"));
    assert_eq!(57, raw_moveset::pos_str2int(&"b8"));

    assert_eq!(63, raw_moveset::pos_str2int(&"h8"));
    assert_eq!(62, raw_moveset::pos_str2int(&"g8"));
}

#[test]
fn test_bishop_moves() {
    {
        let bishop_pos = a2i([4, 4]);
        let mut test_vec = vec![
            a2i([5, 5]),
            a2i([6, 6]),
            a2i([7, 7]),
            a2i([3, 3]),
            a2i([2, 2]),
            a2i([1, 1]),
            a2i([0, 0]),
            a2i([5, 3]),
            a2i([6, 2]),
            a2i([7, 1]),
            a2i([3, 5]),
            a2i([2, 6]),
            a2i([1, 7]),
        ];
        let mut res_vec = raw_moveset::bishop_moves(bishop_pos);

        test_vec.sort();
        res_vec.sort();

        assert_eq!(test_vec, res_vec);
    }
    {
        let bishop_pos = a2i([7, 7]);
        let mut test_vec = vec![
            a2i([6, 6]),
            a2i([5, 5]),
            a2i([4, 4]),
            a2i([3, 3]),
            a2i([2, 2]),
            a2i([1, 1]),
            a2i([0, 0]),
        ];
        let mut res_vec = raw_moveset::bishop_moves(bishop_pos);

        test_vec.sort();
        res_vec.sort();

        assert_eq!(test_vec, res_vec);
    }
}

#[test]
fn test_knight_moves() {
    {
        let knight_pos = a2i([4, 4]);
        let mut test_vec = vec![
            a2i([6, 5]),
            a2i([6, 3]),
            a2i([2, 5]),
            a2i([2, 3]),
            a2i([5, 6]),
            a2i([3, 6]),
            a2i([5, 2]),
            a2i([3, 2]),
        ];
        let mut res_vec = raw_moveset::knight_moves(knight_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }

    {
        let knight_pos = a2i([6, 6]);
        let mut test_vec = vec![a2i([4, 7]), a2i([4, 5]), a2i([7, 4]), a2i([5, 4])];
        let mut res_vec = raw_moveset::knight_moves(knight_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }
}

#[test]
fn test_rook_moves() {
    {
        let rook_pos = a2i([4, 4]);
        let mut test_vec = vec![
            a2i([5, 4]),
            a2i([6, 4]),
            a2i([7, 4]),
            a2i([3, 4]),
            a2i([2, 4]),
            a2i([1, 4]),
            a2i([0, 4]),
            a2i([4, 0]),
            a2i([4, 1]),
            a2i([4, 2]),
            a2i([4, 3]),
            a2i([4, 5]),
            a2i([4, 6]),
            a2i([4, 7]),
        ];
        let mut res_vec = raw_moveset::rook_moves(rook_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }

    {
        let rook_pos = a2i([7, 7]);
        let mut test_vec = vec![
            a2i([6, 7]),
            a2i([5, 7]),
            a2i([4, 7]),
            a2i([3, 7]),
            a2i([2, 7]),
            a2i([1, 7]),
            a2i([0, 7]),
            a2i([7, 0]),
            a2i([7, 1]),
            a2i([7, 2]),
            a2i([7, 3]),
            a2i([7, 4]),
            a2i([7, 5]),
            a2i([7, 6]),
        ];
        let mut res_vec = raw_moveset::rook_moves(rook_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }
}

#[test]
fn test_queen_moves() {
    // moveset of queen actually are a superset of bot queen and rook
    let queen_pos = a2i([4, 4]);
    let mut rook_vec = vec![
        a2i([5, 4]),
        a2i([6, 4]),
        a2i([7, 4]),
        a2i([3, 4]),
        a2i([2, 4]),
        a2i([1, 4]),
        a2i([0, 4]),
        a2i([4, 0]),
        a2i([4, 1]),
        a2i([4, 2]),
        a2i([4, 3]),
        a2i([4, 5]),
        a2i([4, 6]),
        a2i([4, 7]),
    ];
    // now lets append bishop moveset
    rook_vec.append(&mut vec![
        a2i([5, 5]),
        a2i([6, 6]),
        a2i([7, 7]),
        a2i([3, 3]),
        a2i([2, 2]),
        a2i([1, 1]),
        a2i([0, 0]),
        a2i([5, 3]),
        a2i([6, 2]),
        a2i([7, 1]),
        a2i([3, 5]),
        a2i([2, 6]),
        a2i([1, 7]),
    ]);

    let mut queen_moveset = raw_moveset::queen_moves(queen_pos);
    rook_vec.sort();
    queen_moveset.sort();
    assert_eq!(queen_moveset, rook_vec);
}

#[test]
fn test_king_moves() {
    {
        let king_pos = a2i([4, 4]);
        let mut test_vec = vec![
            a2i([5, 3]),
            a2i([5, 4]),
            a2i([5, 5]),
            a2i([4, 3]),
            a2i([4, 5]),
            a2i([3, 3]),
            a2i([3, 4]),
            a2i([3, 5]),
        ];
        let mut res_vec = raw_moveset::king_moves(king_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }

    {
        let king_pos = a2i([7, 7]);
        let mut test_vec = vec![a2i([6, 7]), a2i([6, 6]), a2i([7, 6])];
        let mut res_vec = raw_moveset::king_moves(king_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }
}

#[test]
fn test_pawn_moves() {
    {
        let pawn_pos = a2i([4, 4]);
        let mut test_vec = vec![a2i([4, 5])];
        let mut res_vec = raw_moveset::pawn_moves(pawn_pos, true);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);

        let mut res_vec = raw_moveset::pawn_moves(pawn_pos, false);
        let mut test_vec = vec![a2i([4, 3])];
        res_vec.sort();
        test_vec.sort();
        assert_eq!(test_vec, res_vec);
    }
}
