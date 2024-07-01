use icfpc2024::*;
use std::io::prelude::*;

fn main() {
    loop {
        let (_, value) = eval::input_eval();
        println!("{}", value);
    }
}
