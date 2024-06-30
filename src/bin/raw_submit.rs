use icfpc2024::*;
use std::io::Read;

use std::thread;
use std::time::Duration;

use icfpc2024::communicate;

fn decode(c: char) -> char {
    // TODO: make it a constnat
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars[c as usize - 33];
}

fn encode(c: char) -> char {
    // TODO: make it a constant
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    let index = chars.iter().position(|&x| x == c).unwrap();
    return (index + 33) as u8 as char;
}

fn request(input: &str) -> anyhow::Result<String> {
    eprintln!("--------------------------------------------------------------------------------");
    let text = input;
    eprintln!("Encoded request:\n{}\n", &text);

    let body = communicate(text.to_string())?;
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw response:\n{}\n", body);

    let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Decoded response:\n{}\n", decoded_text);
    Ok(decoded_text)
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
    }

    request(&input)?;

    Ok(())
}
