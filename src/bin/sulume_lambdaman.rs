use std::{collections::HashMap, sync::OnceLock};

use anyhow;
use clap::Parser;
use num::{BigUint, Zero};

#[derive(Parser, Clone, Debug)]
struct Args {
    #[clap(short, long, default_value = "2000")]
    length: usize,
    #[clap(short, long, default_value = "1002887")]
    base: u64,
    #[clap(short, long, default_value = "RDLU")]
    init: String,
    #[clap(short, long, default_value = "0")]
    problem_id: u32,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    let mut s = args.init.bytes().collect::<Vec<_>>();
    while s.len() < args.length {
        let r = str_to_int(&s) % args.base;
        let i: u32 = (&r % BigUint::from(4u32)).try_into().unwrap();
        s.push(s[i as usize]);
    }
    let s = String::from_utf8(s)?;
    let last = str_to_int(s.as_bytes()) % &BigUint::from(s.len());
    print!(
        r#"
B.
    "solve lambdaman{problem_id} "
    B$
        B$
            Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
            Lf Lx
            ?
                B=
                    B%
                          U# vx
                          {base}
                    {last}
                ""
                B$
                    Ls
                        B.
                            vs
                            B$
                                vf
                                    vs
                    B.
                        vx
                        BT
                            1
                            BD 
                                B%
                                    B%
                                        U# vx
                                        {base}
                                    4
                                vx
        "{init}"
"#,
        problem_id = args.problem_id,
        base = args.base,
        last = last,
        init = args.init,
    );
    eprintln!("steps: {}", s);
    eprintln!("last: {}", last);
    Ok(())
}

const CHARS: &[u8; 94] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

fn str_to_int(s: &[u8]) -> BigUint {
    let mut v = BigUint::zero();
    let modulus = &BigUint::from(s.len());
    for c in s {
        v *= modulus;
        static CHARS_MAP: OnceLock<HashMap<u8, usize>> = OnceLock::new();
        v += *CHARS_MAP
            .get_or_init(|| HashMap::from_iter(CHARS.iter().enumerate().map(|(i, &c)| (c, i))))
            .get(&c)
            .unwrap();
    }
    v
}

fn int_to_str(i: &BigUint) -> Vec<u8> {
    let mut s = Vec::new();
    let modulus = &BigUint::from(s.len());
    let mut i = i.clone();
    while !i.is_zero() {
        let x: u32 = (&i % modulus).try_into().unwrap();
        s.push(CHARS[x as usize]);
        i %= modulus;
    }
    return s;
}
