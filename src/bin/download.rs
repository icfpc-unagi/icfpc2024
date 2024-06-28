use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;

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

fn echoeval(input: &str) -> anyhow::Result<String> {
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw request:\n{}\n", &input);

    let url = "https://boundvariable.space/communicate";
    let client = Client::new();

    let res = client
        .post(url)
        .body(r"B. S%#(/} ".to_string() + input)
        .header("Authorization", icfpc2024::get_bearer()?)
        .send()?;

    let body = res.text()?;
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw response:\n{}\n", body);

    let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Decoded response:\n{}\n", decoded_text);

    let suffix = "\nYou scored some points for using the echo service!\n";
    assert!(decoded_text.ends_with(suffix));
    let decoded_text = decoded_text[..decoded_text.len() - suffix.len()].to_owned();

    Ok(decoded_text)
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
        .header("Authorization", icfpc2024::get_bearer()?)
        .send()?;

    let body = res.text()?;
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw response:\n{}\n", body);

    if body.starts_with("B") {
        thread::sleep(Duration::from_secs(3));
        echoeval(&body)
    } else {
        let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
        eprintln!(
            "--------------------------------------------------------------------------------"
        );
        eprintln!("Decoded response:\n{}\n", decoded_text);
        Ok(decoded_text)
    }
}

fn main() -> anyhow::Result<()> {
    for i in 25..26 {
        let input = format!("get spaceship{}", i);
        let output = request(&input)?;
        std::fs::write(format!("./input/spaceship/spaceship{}.txt", i), output)?;
        thread::sleep(Duration::from_secs(3));
    }

    /*
    for i in 21..22 {
        let input = format!("get lambdaman{}", i);
        let output = request(&input)?;
        std::fs::write(format!("./input/lambdaman/lambdaman{}.txt", i), output)?;
        thread::sleep(Duration::from_secs(3));
    }
    */

    Ok(())
}
