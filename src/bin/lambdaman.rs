use clap::Parser;
use icfpc2024::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    problem_id: i64,

    #[arg(short, long)]
    moves: String,
}

fn main() {
    let args = Args::parse();
    let board = lambdaman::simulate_with_problem_id(args.problem_id, args.moves.as_str());
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}
