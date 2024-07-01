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
            program.clear();
            match eval::transpile(&(term)) {
                Ok(transpiled) => {
                    eprintln!("Transpiled ({}): {}", transpiled.len(), transpiled);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    let result = eval::prettify(&term);
                    eprintln!("Prettified:\n{}", result.0.trim());
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
        }
    }
}
