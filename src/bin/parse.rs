use icfpc2024::eval::*;
use std::io::prelude::*;

use itertools::Itertools;
fn main() {
    for line in std::io::stdin().lock().lines() {
        let line: String = line.unwrap();
        let line = line.trim();

        let s = line;
        let tokens: Vec<Vec<u8>> = s
            .split_whitespace()
            .map(|s| s.bytes().collect_vec())
            .collect::<Vec<_>>();
        let mut p = 0;
        let mut binders = vec![];
        let root = parse(&tokens, &mut p, &mut binders);

        dbg!(&root);
    }
}
