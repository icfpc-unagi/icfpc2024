use clap::Parser;
use reqwest::blocking::Client;
use std::fs;
use std::io::prelude::*;
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

fn request(input: &str) -> anyhow::Result<String> {
    // eprintln!("--------------------------------------------------------------------------------");
    // eprintln!("Raw request:\n{}\n", &input);

    let text = "S".to_owned() + &input.chars().map(encode).collect::<String>();

    // eprintln!("--------------------------------------------------------------------------------");
    // eprintln!("Encoded request:\n{}\n", &text);

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

    let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Decoded response:\n{}\n", decoded_text);
    Ok(decoded_text)
}

/// A simple program to send file contents as requests
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to read files from
    #[arg(short, long)]
    directory: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for entry in std::fs::read_dir(&args.directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "txt" {
                    if let Some(file_name) = path.file_stem() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if let Ok(id) = file_name_str.parse::<i32>() {
                                let mut file = std::fs::File::open(&path)?;
                                let mut contents = String::new();
                                file.read_to_string(&mut contents)?;
                                let contents = contents.replace(&['\n', ' '][..], "");

                                if contents.len() >= 1000000 {
                                    println!("Skipping {} because it's too long", path.display());
                                    continue;
                                }

                                let response =
                                    request(&format!("solve spaceship{} {}", id + 1, &contents))?;
                                println!("Response for {}: {}", path.display(), response);
                                thread::sleep(Duration::from_secs(4));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
