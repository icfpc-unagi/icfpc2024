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
    let mut program = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    while handle.read_line(&mut buffer).unwrap() > 0 {
        // for line in std::io::stdin().lock().lines() {
        program += buffer.trim_end();
        buffer.clear();
        program += "\n";
        if program.trim().ends_with(';') {
            eprintln!("Evaluating...");
            let term = program.trim().trim_end_matches(';');
            let transpiled = eval::transpile(&(term)).unwrap();
            eprintln!("Transpiled ({}): {}", transpiled.len(), transpiled);
            match eval::eval(term) {
                Ok(eval::Value::Str(s)) => {
                    let message = String::from_utf8(s.clone()).unwrap();
                    eprintln!("Evaluated: {}", &message);
                    let header = format!("solve lambdaman{} ", args.problem_id);
                    let (encoded_header, moves) = match message.strip_prefix(&header) {
                        Some(moves) => (String::new(), moves),
                        None => (format!("B. S{} ", encode_str(&header)), message.as_str()),
                    };
                    let transpiled = eval::transpile(&(encoded_header + term)).unwrap();
                    eprintln!("Transpiled ({}): {}", transpiled.len(), transpiled);

                    let board = lambdaman::simulate_with_problem_id(args.problem_id, moves);
                    for row in board {
                        println!("{}", row.iter().collect::<String>());
                    }
                }
                Ok(x) => {
                    eprintln!("Expected string, got {:?}", x);
                }
                Err(err) => {
                    eprintln!("Error: {}: {}", err, program);
                }
            }
            program.clear();
        }
    }

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
