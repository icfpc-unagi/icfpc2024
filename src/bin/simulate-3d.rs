use clap::Parser;
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

fn echoeval(input: &str) -> anyhow::Result<String> {
    //eprintln!("--------------------------------------------------------------------------------");
    //eprintln!("Raw request:\n{}\n", &input);

    let body = communicate(r"B. S%#(/} ".to_string() + input)?;

    //eprintln!("--------------------------------------------------------------------------------");
    //eprintln!("Raw response:\n{}\n", body);

    let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
    //eprintln!("--------------------------------------------------------------------------------");
    //eprintln!("Decoded response:\n{}\n", decoded_text);

    let suffix = "\nYou scored some points for using the echo service!\n";
    assert!(decoded_text.ends_with(suffix));
    let decoded_text = decoded_text[..decoded_text.len() - suffix.len()].to_owned();

    Ok(decoded_text)
}

fn request(input: &str) -> anyhow::Result<String> {
    //eprintln!("--------------------------------------------------------------------------------");
    //eprintln!("Raw request:\n{}\n", &input);

    let text = "S".to_owned() + &input.chars().map(encode).collect::<String>();

    //eprintln!("--------------------------------------------------------------------------------");
    //eprintln!("Encoded request:\n{}\n", &text);

    let body = communicate(text.to_string())?;
    //eprintln!("--------------------------------------------------------------------------------");
    //eprintln!("Raw response:\n{}\n", body);

    if body.starts_with("B") {
        thread::sleep(Duration::from_secs(3));
        echoeval(&body)
    } else {
        let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
        /*
        eprintln!(
            "--------------------------------------------------------------------------------"
        );
        eprintln!("Decoded response:\n{}\n", decoded_text);
        */
        Ok(decoded_text)
    }
}

/// A simple program to send file contents as requests
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to read files from
    #[arg(long)]
    board: String,

    #[arg(short, long)]
    a: i32,

    #[arg(short, long)]
    b: i32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    dbg!(&args);

    let mut board = String::new();
    std::fs::File::open(&args.board)?.read_to_string(&mut board)?;
    if board.ends_with('\n') {
        board.pop();
    }

    loop {
        let ret = request(&format!("test 3d {} {}\n{}", args.a, args.b, &board))?;

        let ret = ret.trim().replace("&lt;", "<").replace("&gt;", ">");
        println!("{}", ret);

        let suffix = "Crashed: TickLimitExceeded";
        if !ret.ends_with(suffix) {
            return Ok(());
        }
        let ret = ret[..ret.len() - suffix.len()].to_owned();
        if let Some(last_part) = ret.rsplit(']').next() {
            // println!("Result: {}", last_part);
            board = last_part.to_owned();
        } else {
            println!("No matching pattern found");
            return Ok(());
        }
    }
}
