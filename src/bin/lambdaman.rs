use clap::Parser;
use icfpc2024::*;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    problem_id: i64,
}

fn main() {
    let args = Args::parse();
    let mut moves = String::new();
    for line in std::io::stdin().lock().lines() {
        moves += &line.unwrap();
    }
    let board = lambdaman::simulate_with_problem_id(args.problem_id, moves.as_str());
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}
