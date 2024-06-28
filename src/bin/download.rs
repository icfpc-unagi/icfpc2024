use icfpc2024::encryption::get_bearer;
use reqwest::blocking::Client;
use std::io::Read;

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
    eprintln!("Raw request:\n{}\n", &input);

    let text = "S".to_owned() + &input.chars().map(encode).collect::<String>();

    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Encoded request:\n{}\n", &text);

    let url = "https://boundvariable.space/communicate";
    let client = Client::new();

    let res = client
        .post(url)
        .body(text.to_string())
        .header(
            "Authorization",
            // get_bearer(),
            "Bearer 1b2a9024-2287-4eac-a58f-66a33726e529",
        )
        .send()?;

    let body = res.text()?;
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw response:\n{}\n", body);

    let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Decoded response:\n{}\n", decoded_text);

    Ok(decoded_text)
}

fn main() -> anyhow::Result<()> {
    /*
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
    }
    // request(&input)?;
    */

    for i in 11..21 {
        let input = format!("get lambdaman{}", i);
        let output = request(&input)?;
        std::fs::write(
            format!(
                "/Users/takiba/Dropbox/ICFPC2024/lambdaman/lambdaman{}.txt",
                i
            ),
            output,
        )?;
    }

    Ok(())
}
