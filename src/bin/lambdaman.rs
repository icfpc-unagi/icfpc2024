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
    eprintln!("Problem ID: {}", args.problem_id);

    loop {
        let (program, value) = eval::input_eval();
        match value {
            eval::Value::Str(s) => {
                let message = String::from_utf8(s.clone()).unwrap();
                eprintln!("Evaluated: {}", &message);

                // "solve lambdamanXX " から始まるか確認
                let header = format!("solve lambdaman{} ", args.problem_id);
                let (encoded_header, moves) = match message.strip_prefix(&header) {
                    Some(moves) => (String::new(), moves),
                    None => (format!("B. S{} ", encode_str(&header)), message.as_str()),
                };
                // なければつけたバージョンも表示
                if !encoded_header.is_empty() {
                    let transpiled = eval::transpile(&(encoded_header + &program)).unwrap();
                    eprintln!(
                        "Transpiled with solve ({}): {}",
                        transpiled.len(),
                        transpiled
                    );
                }

                let board = lambdaman::simulate_with_problem_id(args.problem_id, moves);
                for row in board {
                    println!("{}", row.iter().collect::<String>());
                }
            }
            x => {
                eprintln!("Expected string, got {:?}", x);
            }
        }
    }
}
