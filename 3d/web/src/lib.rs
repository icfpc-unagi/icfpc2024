#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ret {
    sim: tools::Sim,
}

#[wasm_bindgen]
impl Ret {
    pub fn get_score(&self) -> i64 {
        self.sim.score
    }
    pub fn get_err(&self) -> String {
        self.sim.err.clone()
    }
    pub fn vis(&self, t: usize) -> String {
        tools::vis(&self.sim, t)
    }
    pub fn get_t(&self, t: usize) -> usize {
        self.sim.log[t].0
    }
    pub fn get_ret(&self) -> String {
        self.sim.ret.to_string()
    }
    pub fn get_max_turn(&self) -> i32 {
        self.sim.log.len() as i32 - 1
    }
}

#[wasm_bindgen]
pub fn parse(input: String, output: String) -> Ret {
    let input = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    match tools::parse_output(&output) {
        Ok(out) => Ret {
            sim: tools::compute_score(&out, &input),
        },
        Err(err) => Ret {
            sim: tools::Sim {
                score: 0,
                err,
                ret: tools::P::Empty,
                log: vec![],
            },
        },
    }
}
