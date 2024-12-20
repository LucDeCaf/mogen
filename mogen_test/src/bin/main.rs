use std::{cmp::Ordering, collections::HashSet};

use clap::{Parser, Subcommand};
use mogen::{
    board::{color::Color, piece::Piece, r#move::Move, square::Square, Board},
    MoveGen,
};
use mogen_test::perft;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short, long)]
    fen: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    Perft {
        #[arg(short, long, default_value = "5")]
        depth: u8,
    },
    Divide {
        #[arg(short, long, default_value = "5")]
        depth: u8,
    },
    Compare {
        #[arg(short, long, default_value = "5")]
        depth: u8,
    },
    Print,
}

fn main() {
    let cli = Cli::parse();

    let board = match cli.fen {
        Some(fen) => Board::from_fen(&fen).unwrap(),
        None => Board::default(),
    };

    match cli.command {
        Command::Perft { depth } => {
            let total = perft::perft(&board, depth);
            println!("---- START PERFT RESULTS ----");
            println!("depth = {depth}: {total}");
            println!("---- END PERFT RESULTS ----");
        }

        Command::Divide { depth } => {
            let (results, total) = perft::divide(&board, depth);

            println!("---- START DIVIDE RESULTS ----");

            for res in results {
                println!("{}: {}", res.0, res.1);
            }
            println!("Total node count: {total}");

            println!("---- END DIVIDE RESULTS ----");
        }

        Command::Compare { depth } => {
            let results = perft::compare(&board, depth);

            println!("---- START COMPARE RESULTS ----\n");

            let mut move_set = HashSet::new();
            for k in results.stockfish_results.keys() {
                move_set.insert(*k);
            }
            for k in results.mogen_results.keys() {
                move_set.insert(*k);
            }

            let mut moves = move_set.into_iter().collect::<Vec<Move>>();
            moves.sort_unstable();

            println!(
                "{: <8} {: <12} {: <12} Symbol\n",
                "Move", "Mogen", "Stockfish"
            );

            for mv in moves {
                let stockfish = match results.stockfish_results.get(&mv) {
                    Some(count) => *count,
                    None => 0,
                };
                let mogen = match results.mogen_results.get(&mv) {
                    Some(count) => *count,
                    None => 0,
                };

                let symbol = match mogen.cmp(&stockfish) {
                    Ordering::Greater => "+",
                    Ordering::Less => "-",
                    Ordering::Equal => "",
                };

                println!(
                    "{: <8} {: <12} {: <12} {}",
                    mv.to_string(),
                    if mogen == 0 {
                        String::new()
                    } else {
                        mogen.to_string()
                    },
                    if stockfish == 0 {
                        String::new()
                    } else {
                        stockfish.to_string()
                    },
                    symbol
                );
            }

            let diff = results.mogen_total as i32 - results.stockfish_total as i32;
            println!("\nNode count difference: {}\n", diff);

            println!("---- END COMPARE RESULTS ----");
        }
        Command::Print => {
            let mg = MoveGen::new();

            println!("{:?}", board.active_color);

            let mut moves = Vec::new();
            mg.bishop_moves(&board, Color::White, Square::F1, &mut moves);

            println!("{}", moves[0]);
        }
    }
}
