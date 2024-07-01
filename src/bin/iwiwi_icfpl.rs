use icfpc2024::eval::*;
// use std::io::prelude::*;
use core::num;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

fn encode(c: char) -> char {
    // TODO: make it a constant
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    let index = chars.iter().position(|&x| x == c).unwrap();
    return (index + 33) as u8 as char;
}

fn encode_str(s: &String) -> String {
    let s2 = s.chars().map(encode).collect::<String>();
    format!("S{s2}")
}

fn encode_i(inp: BigInt) -> String {
    let mut i = inp;
    let zero = BigInt::from(0);
    let mut s = String::new(); // 空の文字列を初期化

    while i > zero {
        let r = (i.clone() % 94u32).to_u32().unwrap();
        s = format!("{}{}", decode_from_i(r), s);
        i /= 94u32;
    }

    if s == "" {
        s = "!".to_string();
    }

    format!("I{}", s)
}

fn decode_from_i(c: u32) -> char {
    // TODO: make it a constnat
    //println!("{}", c);
    //let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return (c + 33) as u8 as char;
}

fn main() {
    let prefix = encode_str(&"solve lambdaman9 ".to_string());

    let all = format!("B. {prefix} B$ Lf B$ vf B. B. B$ vf SL B$ vf SF S> B$ Lf Ls B$ vf B$ vf B$ vf vs Ls B. B. vs vs B. vs vs");
    let all = format!("B. {prefix} B$ Lf B$ vf B. B. B$ vf SLL B$ vf SFF S> B$ Lf Ls B$ vf B$ vf B$ vf vs Ls B. B. vs vs vs"); // 縦が足りない

    // let all = format!("B. {prefix} B$ Lf B$ vf B. B. B$ vf SL B$ vf SF S> B$ Lf Ls B$ vf B$ vf B$ vf B$ vf vs Ls B. B. vs vs vs");
    // let all = format!("B. {prefix} B$ Lf B$ vf B. B. B$ vf SL B$ vf SF S> B$ Lf Ls B$ vf B$ vf B$ vf B$ vf B$ vf B$ vf vs Ls B. vs vs");
    eprintln!("{}", eval(&all).unwrap());
    eprintln!("{}", all);
    dbg!(all.len());

    if true {
        debug_parse("B. S3/,6%},!-\"$!-!.]} B$ Lf B$ vf B. B$ vf SOL B$ vf S>F B$ Lf Ls B$ vf B$ vf B$ vf B$ vf vs Ls B. B. vs vs B. vs vs");
        return;
    }

    if true {
        //let t = encode_i(101.into());
        let t = encode_str(&"LLLLLLLLLL".to_string());
        dbg!(&t);
        eprintln!("{}", eval(&t).unwrap());
        return;
    }

    // IS = Int(50)
    // SL = Str("R")
    // SF = Str("L")
    // S> = Str("D")
    let cgen = "Lx ? B< vx IS SL ? B< vx I\"' SF S>";

    let y = "Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx";
    // TODO vx -> vx % 101
    // I\"( = INt(101)
    let max = encode_i(10000.into());
    let mo = encode_i(101.into());
    let f = format!(r#"B$ {y} Lf Lx ? B< vx {max} B. B$ {cgen} B% vx {mo} B$ vf B+ vx I" S"#);
    // I@w = Int(3000)
    let hoge = format!("B$ {f} {}", encode_i(0.into()));

    dbg!(hoge.len());
    eprintln!("{}", &hoge);
    // eprintln!("{}", eval(&hoge));

    eprintln!(
        "{}",
        format!(
            "B. {} {}",
            encode_str(&"solve lambdaman9 ".to_string()),
            hoge
        )
    );
    /*

    // SL = Str("R")
    let y = "Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx";
    let f = format!(r#"B$ {y} Lf Lx ? B> vx I! B. SL B$ vf B- vx I" S"#);
    let r100 = format!("B$ {f} I\"'");

    let f = format!(r#"B$ {y} Lf Lx ? B> vx I! B. SF B$ vf B- vx I" S"#);
    let l100 = format!("B$ {f} I\"'");

    // S> = Str("D")
    let r100l100d = format!("B. B. {r100} {l100} S>");

    let f = format!(r#"B$ {y} Lf Lx ? B> vx I! B. {r100l100d} B$ vf B- vx I" S"#);
    let hoge = format!("B$ {f} I\"'");

    // I& = Int(5)
    //let hoge = r100l100d;
    eprintln!("{}", &hoge);
    eprintln!("{}", eval(&hoge));

    eprintln!(
        "{}",
        format!(
            "B. {} {}",
            encode_str(&"solve lambdaman9 ".to_string()),
            hoge
        )
    );

    dbg!(y.len());

    return;

    // debug_parse(&f);

    // println!("{}", Node::Const(Value::Str(b"LLLLL".to_vec())));

    // dbg!(eval(&format!("B$ {f} I&")), Value::Int(120.into()));
    */
}
