use icfpc2024::encryption::{decrypt, encrypt};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <text_to_encrypt>", args[0]);
        std::process::exit(1);
    }

    let encrypted_text = encrypt(args[1].as_str());
    println!("Encrypted: {}", encrypted_text);

    let decrypted_text = decrypt(&encrypted_text);
    println!("Decrypted: {}", decrypted_text);
}
