use clap::{Parser, Subcommand};
use mogen::board::Board;
use mogen_test::perft;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

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

            println!("---- START DIVIDE RESULTS ----");

            // TODO: Implement extra/missing move detection, check for move ordering, check if results match

            println!("---- END DIVIDE RESULTS ----");
        }
    }
}
