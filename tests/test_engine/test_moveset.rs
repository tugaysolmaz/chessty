use cheest::{
    engine::{moveset, raw_moveset::pos_arr2int as a2i},
    table::Table,
};

fn empty_table() -> Table {
    Table::new_empty()
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
        let mut res_vec = moveset::bishop_moves(&mut empty_table(), bishop_pos);

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
        let mut res_vec = moveset::bishop_moves(&mut empty_table(), bishop_pos);

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
        let mut res_vec = moveset::knight_moves(&mut empty_table(), knight_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }

    {
        let knight_pos = a2i([6, 6]);
        let mut test_vec = vec![a2i([4, 7]), a2i([4, 5]), a2i([7, 4]), a2i([5, 4])];
        let mut res_vec = moveset::knight_moves(&mut empty_table(), knight_pos);
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
        let mut res_vec = moveset::rook_moves(&mut empty_table(), rook_pos);
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
        let mut res_vec = moveset::rook_moves(&mut empty_table(), rook_pos);
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

    let mut queen_moveset = moveset::queen_moves(&mut empty_table(), queen_pos);
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
        let mut res_vec = moveset::king_moves(&mut empty_table(), king_pos);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);
    }

    {
        let king_pos = a2i([7, 7]);
        let mut test_vec = vec![a2i([6, 7]), a2i([6, 6]), a2i([7, 6])];
        let mut res_vec = moveset::king_moves(&mut empty_table(), king_pos);
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
        let mut res_vec = moveset::pawn_moves(&mut empty_table(), pawn_pos, true);
        test_vec.sort();
        res_vec.sort();
        assert_eq!(test_vec, res_vec);

        let mut res_vec = moveset::pawn_moves(&mut empty_table(), pawn_pos, false);
        let mut test_vec = vec![a2i([4, 3])];
        res_vec.sort();
        test_vec.sort();
        assert_eq!(test_vec, res_vec);
    }
}
