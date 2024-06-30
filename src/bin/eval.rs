use icfpc2024::eval::*;
use std::io::prelude::*;

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        match eval(line) {
            Ok(result) => {
                println!("{}", result);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
