use std::io::Read;

use reqwest::blocking::Client;

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
        .header(
            "Authorization",
            "Bearer 1b2a9024-2287-4eac-a58f-66a33726e529",
        )
        .send()?;

    let body = res.text()?;
    println!("--------------------------------------------------------------------------------");
    println!("Raw response:\n{}\n", body);

    Ok(())
}
