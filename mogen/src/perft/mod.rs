use crate::{
    board::{r#move::Move, Board},
    MoveGen,
};

pub fn perft_inner(board: &Board, depth: u8, move_gen: &MoveGen) -> u32 {
    let mut moves = Vec::new();

    move_gen.pseudolegal_moves(board, &mut moves);
    let mut count = 0;

    if depth == 0 {
        return 1;
    }

    if depth == 1 {
        return moves.len() as u32;
    }

    for mv in &moves {
        let board = board.make_move(*mv);
        count += perft_inner(&board, depth - 1, move_gen);
    }

    count
}

pub fn perft(board: &Board, depth: u8) -> u32 {
    let move_gen = MoveGen::new();
    perft_inner(board, depth, &move_gen)
}

pub fn divide_inner(board: &Board, depth: u8, move_gen: &MoveGen) -> Vec<(u32, Move)> {
    let mut results = Vec::new();

    let mut moves = Vec::new();
    move_gen.pseudolegal_moves(board, &mut moves);

    for mv in moves {
        let perft_result = perft_inner(board, depth - 1, move_gen);
        let result = (perft_result, mv);

        results.push(result);
    }

    results
}

pub fn divide(board: &Board, depth: u8) -> Vec<(u32, Move)> {
    let move_gen = MoveGen::new();
    divide_inner(board, depth, &move_gen)
}
