use icfpc2024::eval::*;
use std::io::prelude::*;

fn main() {
    let mut program = String::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        program += line;
        program += "\n";
        if program.trim().ends_with(';') {
            match eval(program.trim().trim_end_matches(';')) {
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
