use icfpc2024::*;
use std::io::prelude::*;

fn main() {
    eprintln!("Enter a program to evaluate (end with ';'):");
    let mut program = String::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        program += line;
        program += "\n";
        if program.trim().ends_with(';') {
            let term = program.trim().trim_end_matches(';').to_owned();
            match eval::transpile(&(term)) {
                Ok(transpiled) => {
                    eprintln!("Transpiled ({}): {}", transpiled.len(), transpiled);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    match eval::prettify(&term) {
                        Ok(pretty) => {
                            eprintln!("Prettified:\n{}", pretty);
                        }
                        Err(err) => {
                            eprintln!("Failed to prettify: {}", err);
                        }
                    }
                    continue;
                }
            }
            match eval::eval(&term) {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
            program.clear();
        }
    }
}
