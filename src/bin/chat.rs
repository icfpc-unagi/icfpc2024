use icfpc2024::encryption::get_bearer;
use icfpc2024::*;
use reqwest::blocking::Client;
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

    let url = "https://boundvariable.space/communicate";
    let client = Client::new();

    let res = client
        .post(url)
        .body(text.to_string())
        .header("Authorization", get_bearer())
        .send()?;

    let body = res.text()?;
    println!("--------------------------------------------------------------------------------");
    println!("Raw response:\n{}\n", body);

    let decoded = decode(&body);
    println!("--------------------------------------------------------------------------------");
    println!("Decoded response:\n{}\n", decoded);

    Ok(())
}
