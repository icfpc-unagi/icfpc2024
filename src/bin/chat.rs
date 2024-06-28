use icfpc2024::*;
use std::io::Read;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
    }

    println!("--------------------------------------------------------------------------------");
    println!("Raw request:\n{}\n", &input);

    let text = "S".to_owned() + &encode_str(&input);

    println!("--------------------------------------------------------------------------------");
    println!("Encoded request:\n{}\n", &text);

    let body = communicate(text.to_string())?;
    println!("--------------------------------------------------------------------------------");
    println!("Raw response:\n{}\n", body);

    let decoded = decode(&body);
    println!("--------------------------------------------------------------------------------");
    println!("Decoded response:\n{}\n", decoded);

    Ok(())
}
