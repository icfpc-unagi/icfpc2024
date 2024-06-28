use std::io::Read;

use icfpc2024::*;

fn main() -> anyhow::Result<()> {
    let mut input = r"B. S%#(/} ".to_string();
    std::io::stdin().read_to_string(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
    }

    let body = communicate(input)?;
    println!("--------------------------------------------------------------------------------");
    println!("Raw response:\n{}\n", body);

    let decoded_text = decode(&body);
    // We know "dyn Any" result is actually String
    // let decoded_text = decoded_text.downcast_ref::<String>().unwrap();
    println!("--------------------------------------------------------------------------------");
    println!("Decoded response:\n{}\n", decoded_text);

    Ok(())
}
