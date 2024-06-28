use anyhow::Context;
#[cfg(feature = "reqwest")]
use reqwest::Client;
use std::fmt::Display;

use itertools::Itertools;

#[cfg(feature = "tokio")]
#[cfg(feature = "reqwest")]
pub mod www;

#[cfg(feature = "mysql")]
pub mod sql;

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
    ($($e:expr),*) => { vec![$($e),*] };
    ($($e:expr,)*) => { vec![$($e),*] };
    ($e:expr; $d:expr) => { vec![$e; $d] };
    ($e:expr; $d:expr $(; $ds:expr)+) => { vec![mat![$e $(; $ds)*]; $d] };
}

pub mod eval;

const CHARS: &[u8; 94] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n";

pub fn encode_char(c: char) -> Option<char> {
    let c = c as u8;
    let index = CHARS.iter().find_position(|&x| *x == c)?.0;
    return Some((index + 33) as u8 as char);
}

pub fn decode_char(c: char) -> Option<char> {
    return Some(CHARS[(c as usize).checked_sub(33)?] as char);
}

pub fn encode_str(s: &str) -> String {
    s.chars().flat_map(encode_char).collect()
}

fn decode_str(s: &str) -> String {
    s.chars().flat_map(decode_char).collect()
}

// TODO: Implement with bigint
fn decode_base94(s: &str) -> u128 {
    let mut n = 0;
    for c in s.chars() {
        n = n * 94 + (c as u8 - '!' as u8) as u128;
    }
    n
}

#[cfg(feature = "reqwest")]
pub async fn get_bearer_async() -> anyhow::Result<String> {
    let unagi_password = std::env::var("UNAGI_PASSWORD").context("UNAGI_PASSWORD not set")?;
    let client = Client::new();
    let res = client
        .get(&format!(
            "https://storage.googleapis.com/icfpc2024-data/{}/bearer.txt",
            unagi_password,
        ))
        .send()
        .await
        .context("Failed to get bearer")?;
    res.text()
        .await
        .context("Failed to get bearer")
        .map_err(Into::into)
        .map(|s| format!("Bearer {}", s))
}

#[cfg(all(feature = "reqwest", feature = "tokio"))]
pub fn get_bearer() -> anyhow::Result<String> {
    tokio::runtime::Runtime::new()?.block_on(get_bearer_async())
}

// TODO: Implement with bigint
fn encode_base94(mut n: u128) -> String {
    let mut chars = vec![];
    while n > 0 {
        chars.push((n % 94 + '!' as u128) as u8 as char);
        n /= 94;
    }
    chars.iter().rev().collect()
}

pub fn decode(s: &str) -> Box<dyn Display> {
    let (indicator, rest) = s.split_at(1);
    match indicator {
        "T" => Box::new(true),
        "F" => Box::new(false),
        "I" => Box::new(decode_base94(rest)),
        "S" => Box::new(decode_str(rest)),
        _ => todo!("Unknown indicator {}", indicator),
    }
}

#[cfg(feature = "reqwest")]
pub async fn communicate_async(message: String) -> Result<String, anyhow::Error> {
    Ok(Client::new()
        .post("https://boundvariable.space/communicate")
        .header("Authorization", get_bearer_async().await?)
        .body(message)
        .send()
        .await?
        .text()
        .await?)
}

#[cfg(all(feature = "reqwest", feature = "tokio"))]
pub fn communicate(message: String) -> Result<String, anyhow::Error> {
    tokio::runtime::Runtime::new()?.block_on(communicate_async(message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base94() {
        assert_eq!(encode_base94(1), "\"");
        assert_eq!(encode_base94(1337), "/6");
    }

    #[test]
    fn test_decode_base94() {
        assert_eq!(decode_base94("\""), 1);
        assert_eq!(decode_base94("/6"), 1337);
    }

    #[test]
    fn test_decode_str() {
        assert_eq!(decode_str("B%,,/}Q/2,$_"), "Hello World!");
    }

    #[test]
    fn test_encode_str() {
        assert_eq!(encode_str("Hello World!"), "B%,,/}Q/2,$_");
    }
}
