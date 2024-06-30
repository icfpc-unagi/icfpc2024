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
    for line in std::io::stdin().lock().lines() {
        let line = match line {
            Ok(line) => line.to_string(),
            _ => continue,
        };
        match if line.chars().all(|c| "LRUD".contains(c)) {
            Ok(line)
        } else {
            match eval::eval(&line) {
                Ok(eval::Value::Str(s)) => {
                    eprintln!("Evaluated: {}", String::from_utf8(s.clone()).unwrap());
                    Ok(String::from_utf8(s).unwrap())
                }
                Ok(x) => Err(anyhow::anyhow!("Expected string, got {:?}", x)),
                Err(err) => Err(err),
            }
        } {
            Ok(moves) => {
                let board = lambdaman::simulate_with_problem_id(args.problem_id, moves.as_str());
                for row in board {
                    println!("{}", row.iter().collect::<String>());
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
