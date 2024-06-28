use icfpc2024::encryption::decrypt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <text_to_encrypt>", args[0]);
        std::process::exit(1);
    }

    let decrypted_text = decrypt(args[1].as_str());
    println!("Decrypted: {}", decrypted_text);
}
