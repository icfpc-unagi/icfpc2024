use clap::Parser;
use std::io::Read;


fn decode(c: char) -> Option<char> {
    // TODO: make it a constant
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars.get((c as usize).checked_sub(33)?).copied();
}

fn encode(c: char) -> Option<char> {
    // TODO: make it a constant
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    let index = chars.iter().position(|&x| x == c)?;
    return Some((index + 33) as u8 as char);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    encode: bool,
}

fn main() {
    let args = Args::parse();
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let f = if args.encode { encode } else { decode };
    let s =  s.chars().map(f).filter_map(|x| x).collect::<String>();
    println!("{}", s);
}
