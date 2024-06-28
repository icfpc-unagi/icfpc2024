use std::fmt::Display;

#[cfg(feature = "tokio")]
#[cfg(feature = "reqwest")]
pub mod www;

#[cfg(feature = "mysql")]
pub mod sql;

pub mod encryption;

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

pub fn encode_char(c: char) -> Option<char> {
    // TODO: make it a constant
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    let index = chars.iter().position(|&x| x == c)?;
    return Some((index + 33) as u8 as char);
}

pub fn decode_char(c: char) -> Option<char> {
    // TODO: make it a constant
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars.get((c as usize).checked_sub(33)?).copied();
}

pub fn encode_str(s: &str) -> String {
    s.chars().flat_map(encode_char).collect()
}

pub fn decode_str(s: &str) -> String {
    s.chars().flat_map(decode_char).collect()
}

// TODO: Implement with bigint
pub fn decode_base94(s: &str) -> u128 {
    let mut n = 0;
    for c in s.chars() {
        n = n * 94 + (c as u8 - '!' as u8) as u128;
    }
    n
}

#[test]
fn test_decode_base94() {
    assert_eq!(decode_base94("\""), 1);
    assert_eq!(decode_base94("/6"), 1337);
}

// TODO: Implement with bigint
pub fn encode_base94(mut n: u128) -> String {
    let mut chars = vec![];
    while n > 0 {
        chars.push((n % 94 + '!' as u128) as u8 as char);
        n /= 94;
    }
    chars.iter().rev().collect()
}

#[test]
fn test_encode_base94() {
    assert_eq!(encode_base94(1), "\"");
    assert_eq!(encode_base94(1337), "/6");
}

pub fn decode(s: &str) -> Box<dyn Display> {
    let (indicator, rest) = s.split_at(1);
    match indicator {
        "T" => Box::new(true),
        "F" => Box::new(false),
        "I" => Box::new(decode_base94(rest)),
        "S" => Box::new(decode_str(rest)),
        _ => unimplemented!("Unknown indicator"),
    }
}
