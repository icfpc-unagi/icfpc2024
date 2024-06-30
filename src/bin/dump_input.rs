use std::io::prelude::*;

use icfpc2024::eval::*;

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if !line.is_empty() {
            debug_parse(line);
        }
    }
}

// fn main() {
//     // let x = |v2: i64| { |v3| { v2 } } (String::from("Hello") + &String::from(" World!"))(42);
//     // println!("{}", x);
// }
