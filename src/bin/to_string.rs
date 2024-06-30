use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use std::env;

fn main() {
    // Get first arg.
    let args: Vec<String> = env::args().collect();
    // Check if there are less than 2 args.
    if args.len() < 2 {
        // Print usage.
        eprintln!("Usage: {} <text_to_encrypt>", args[0]);
        // Exit with error.
        std::process::exit(1);
    }
    let mut number = args[1].parse::<BigInt>().unwrap();
    let mut icfp_str: String = "".to_string();
    while number > 0u32.into() {
        let quotient = &number / 94;
        let remainder: BigInt = &number % 94;
        icfp_str.push((remainder + 33u32).to_u8().unwrap() as char);
        number = quotient;
    }
    println!("{}", icfp_str.chars().rev().collect::<String>());
}
