#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GenOption {}

#[wasm_bindgen]
impl GenOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
    pub fn new_default() -> Self {
        Self {}
    }
}

#[wasm_bindgen]
pub fn gen(seed: String, _option: &GenOption) -> String {
    if let Ok(seed) = seed.parse::<u64>() {
        let input = tools::gen(seed);
        input.to_string()
    } else {
        "Seed must be an unsigned 64 bit integer".to_owned()
    }
}

#[wasm_bindgen]
pub struct Ret {
    pub score: i64,
    #[wasm_bindgen(getter_with_clone)]
    pub error: String,
    #[wasm_bindgen(getter_with_clone)]
    pub svg: String,
}

#[wasm_bindgen]
pub struct VisOption {}

#[wasm_bindgen]
impl VisOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
}

#[wasm_bindgen]
pub fn vis(input: String, output: String, t: i32, _option: &VisOption) -> Ret {
    let t = t as usize;
    let input = tools::parse_input(&input);
    match tools::parse_output(&input, &output) {
        Ok(out) => {
            let (score, error, svg) = tools::vis(&input, &out.out[..t]);
            Ret { score, error, svg }
        }
        Err(error) => Ret {
            score: 0,
            error,
            svg: String::new(),
        },
    }
}

#[wasm_bindgen]
pub fn get_max_turn(input: String, output: String) -> i32 {
    let input = tools::parse_input(&input);
    if let Ok(out) = tools::parse_output(&input, &output) {
        out.out.len() as i32
    } else {
        0
    }
}
