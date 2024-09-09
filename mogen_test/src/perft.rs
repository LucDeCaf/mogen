use mogen::{
    board::{r#move::Move, Board},
    MoveGen,
};

pub fn perft(board: &Board, depth: u8) -> u32 {
    if depth == 0 {
        return 1;
    }
    let move_gen = MoveGen::new();
    perft_inner(board, depth, &move_gen)
}

fn perft_inner(board: &Board, depth: u8, move_gen: &MoveGen) -> u32 {
    if depth == 0 {
        return 1;
    }

    let mut moves = Vec::new();
    move_gen.pseudolegal_moves(board, &mut moves);

    if depth == 1 {
        return moves.len() as u32;
    }

    let mut count = 0;

    for mv in moves {
        let board = board.make_move(mv);
        count += perft_inner(&board, depth - 1, move_gen);
    }

    count
}

pub fn divide(board: &Board, depth: u8) -> (Vec<(Move, u32)>, u32) {
    if depth == 0 {
        return (Vec::new(), 1);
    }

    let move_gen = MoveGen::new();

    let mut moves = Vec::new();
    move_gen.pseudolegal_moves(board, &mut moves);

    let mut results = Vec::new();
    let mut total = 0;

    for mv in moves {
        let board = board.make_move(mv);

        let count = perft_inner(&board, depth - 1, &move_gen);
        total += count;

        results.push((mv, count));
    }

    (results, total)
}
