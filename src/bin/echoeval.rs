use std::io::Read;

use reqwest::blocking::Client;

use icfpc2024::decode;

fn main() -> anyhow::Result<()> {
    let mut input = r"B. S%#(/} ".to_string();
    std::io::stdin().read_to_string(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
    }

    let url = "https://boundvariable.space/communicate";
    let client = Client::new();

    let res = client
        .post(url)
        .body(input)
        .header("Authorization", icfpc2024::get_bearer()?)
        .send()?;

    let body = res.text()?;
    println!("--------------------------------------------------------------------------------");
    println!("Raw response:\n{}\n", body);

    let decoded_text = decode(&body);
    // We know "dyn Any" result is actually String
    // let decoded_text = decoded_text.downcast_ref::<String>().unwrap();
    println!("--------------------------------------------------------------------------------");
    println!("Decoded response:\n{}\n", decoded_text);

    Ok(())
}
