use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

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

#[derive(Debug)]
pub struct CompareResult {
    pub stockfish_results: HashMap<Move, u32>,
    pub stockfish_total: u32,
    pub mogen_results: HashMap<Move, u32>,
    pub mogen_total: u32,
}

// TODO: Test if this loads FEN string correctly for Stockfish
pub fn compare(board: &Board, depth: u8) -> CompareResult {
    let mut stockfish = Command::new("stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let fen = board.fen();
    let cmd = format!("uci\nucinewgame\nposition fen {fen}\n");

    let mut stdin = stockfish.stdin.take().unwrap();
    let mut stdout = BufReader::new(stockfish.stdout.take().unwrap());

    // Set stockfish board state
    stdin.write_all(cmd.as_bytes()).unwrap();

    let mut buf = String::new();

    // Read past opening lines
    let mut i = 0;
    loop {
        i += 1;
        if i == 1000 {
            println!("ERROR: Max iterations reached");
            break;
        }

        stdout.read_line(&mut buf).unwrap();
        if buf == "uciok\n" {
            break;
        }
        buf.clear();
    }

    buf.clear();

    // Get stockfish results
    stdin
        .write_all(format!("go perft {depth}\n").as_bytes())
        .unwrap();

    let mut stockfish_results = HashMap::new();
    let stockfish_total;

    loop {
        buf.clear();
        stdout.read_line(&mut buf).unwrap();
        buf = buf.trim().to_string();

        if buf.is_empty() || buf.starts_with("info") {
            continue;
        }

        let mut parts = buf.split(':');
        let tag = parts.next().unwrap().trim();
        let count = parts.next().unwrap().trim();
        let count = count.parse::<u32>().unwrap();

        match Move::try_from(tag) {
            Ok(mv) => {
                stockfish_results.insert(mv, count);
            }
            Err(_) => {
                stockfish_total = count;
                break;
            }
        }
    }

    let (vec_results, mogen_total) = divide(board, depth);

    let mogen_results = {
        let mut map = HashMap::new();
        for (mv, count) in vec_results {
            map.insert(mv, count);
        }
        map
    };

    CompareResult {
        stockfish_results,
        stockfish_total,
        mogen_results,
        mogen_total,
    }
}
