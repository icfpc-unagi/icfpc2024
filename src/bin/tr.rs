use clap::Parser;
use icfpc2024::*;
use std::io::Read;

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
    let f = if args.encode {
        encode_char
    } else {
        decode_char
    };
    let s = s.chars().map(f).filter_map(|x| x).collect::<String>();
    println!("{}", s);
}
