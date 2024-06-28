#![allow(non_snake_case)]

use itertools::Itertools;
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Bool(bool),
    Int(BigInt),
    Str(Vec<u8>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "bool({})", b),
            Value::Int(i) => write!(f, "int({})", i),
            Value::Str(s) => write!(f, "str({})", String::from_utf8_lossy(s)),
        }
    }
}

macro_rules! const_char_array {
    ($s:expr) => {{
        const ARR: [u8; $s.len()] = {
            let mut arr = [0; $s.len()];
            let mut i = 0;
            while i < $s.len() {
                arr[i] = $s.as_bytes()[i];
                i += 1;
            }
            arr
        };
        ARR
    }};
}

const CHARS: [u8; 94] = const_char_array!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n");

fn S(body: &[u8]) -> Value {
    Value::Str(body.iter().map(|&b| CHARS[b as usize - 33]).collect())
}

fn rec(tokens: &[Vec<u8>], p: &mut usize) -> Value {
    let id = tokens[*p][0];
    let body = &tokens[*p][1..];
    *p += 1;
    match id {
        b'T' => {
            assert_eq!(body.len(), 0);
            Value::Bool(true)
        }
        b'F' => {
            assert_eq!(body.len(), 0);
            Value::Bool(false)
        }
        b'I' => {
            let mut val = BigInt::from(0);
            for &b in body {
                val *= 94;
                val += b - 33;
            }
            Value::Int(val)
        }
        b'S' => S(body),
        b'U' => {
            assert_eq!(body.len(), 1);
            match body[0] {
                b'-' => {
                    if let Value::Int(val) = rec(tokens, p) {
                        Value::Int(-val)
                    } else {
                        panic!("negation of non-int");
                    }
                }
                b'!' => {
                    if let Value::Bool(val) = rec(tokens, p) {
                        Value::Bool(!val)
                    } else {
                        panic!("negation of non-bool");
                    }
                }
                b'#' => {
                    if let Value::Str(val) = rec(tokens, p) {
                        let mut v = BigInt::from(0);
                        for b in val {
                            v *= 94;
                            v += CHARS.iter().position(|&c| c == b).unwrap();
                        }
                        Value::Int(v)
                    } else {
                        panic!("length of non-str");
                    }
                }
                b'$' => {
                    if let Value::Int(mut val) = rec(tokens, p) {
                        let mut s = vec![];
                        while val > 0.into() {
                            let v: u8 = (val.clone() % BigInt::from(94)).try_into().unwrap();
                            s.push(v + 33);
                            val /= 94;
                        }
                        s.reverse();
                        S(&s)
                    } else {
                        panic!("stringify of non-int");
                    }
                }
                op => {
                    panic!("unknown op: {}", op as char);
                }
            }
        }
        b'B' => {
            assert_eq!(body.len(), 1);
            match body[0] {
                b'+' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                        _ => panic!("addition of non-int"),
                    }
                }
                b'-' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                        _ => panic!("subtraction of non-int"),
                    }
                }
                b'*' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                        _ => panic!("multiplication of non-int"),
                    }
                }
                b'/' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                        _ => panic!("division of non-int"),
                    }
                }
                b'%' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                        _ => panic!("modulo of non-int"),
                    }
                }
                b'<' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
                        _ => panic!("comparison of non-int"),
                    }
                }
                b'>' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
                        _ => panic!("comparison of non-int"),
                    }
                }
                b'=' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a == b),
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                        (Value::Str(a), Value::Str(b)) => Value::Bool(a == b),
                        _ => panic!("comparison of different types"),
                    }
                }
                b'|' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                        _ => panic!("or of non-bool"),
                    }
                }
                b'&' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                        _ => panic!("and of non-bool"),
                    }
                }
                b'.' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Str(mut a), Value::Str(b)) => {
                            a.extend_from_slice(&b);
                            Value::Str(a)
                        }
                        _ => panic!("concat of non-str"),
                    }
                }
                b'T' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Str(b)) => {
                            let a: usize = a.try_into().unwrap();
                            Value::Str(b[..a].to_vec())
                        }
                        _ => panic!("take of non-int or non-str"),
                    }
                }
                b'D' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    match (a, b) {
                        (Value::Int(a), Value::Str(b)) => {
                            let a: usize = a.try_into().unwrap();
                            Value::Str(b[a..].to_vec())
                        }
                        _ => panic!("drop of non-int or non-str"),
                    }
                }
                b'$' => {
                    let a = rec(tokens, p);
                    let b = rec(tokens, p);
                    unimplemented!()
                }
                op => {
                    panic!("unknown op: {}", op as char);
                }
            }
        }
        b'?' => {
            assert_eq!(body.len(), 0);
            let cond = rec(tokens, p);
            let a = rec(tokens, p);
            let b = rec(tokens, p);
            match cond {
                Value::Bool(true) => a,
                Value::Bool(false) => b,
                _ => panic!("condition is not bool"),
            }
        }
        b'L' => {
            let mut val = BigInt::from(0);
            unimplemented!()
        }
        id => {
            panic!("unknown id: {}", id as char);
        }
    }
}

pub fn eval(s: &str) -> Value {
    let tokens = s
        .split_whitespace()
        .map(|s| s.bytes().collect_vec())
        .collect::<Vec<_>>();
    let mut p = 0;
    let ret = rec(&tokens, &mut p);
    assert_eq!(p, tokens.len());
    ret
}

#[test]
fn test() {
    assert_eq!(eval("I/6"), Value::Int(1337.into()));
    assert_eq!(eval("SB%,,/}Q/2,$_"), Value::Str(b"Hello World!".to_vec()));
    assert_eq!(eval("U- I$"), Value::Int((-3).into()));
    assert_eq!(eval("U! T"), Value::Bool(false));
    assert_eq!(eval("U# S4%34"), Value::Int(15818151.into()));
    assert_eq!(eval("U$ I4%34"), Value::Str(b"test".to_vec()));
    assert_eq!(eval("B+ I# I$"), Value::Int(5.into()));
    assert_eq!(eval("B- I$ I#"), Value::Int(1.into()));
    assert_eq!(eval("B* I$ I#"), Value::Int(6.into()));
    assert_eq!(eval("B/ U- I( I#"), Value::Int((-3).into()));
    assert_eq!(eval("B% U- I( I#"), Value::Int((-1).into()));
    assert_eq!(eval("B< I$ I#"), Value::Bool(false));
    assert_eq!(eval("B> I$ I#"), Value::Bool(true));
    assert_eq!(eval("B= I$ I#"), Value::Bool(false));
    assert_eq!(eval("B| T F"), Value::Bool(true));
    assert_eq!(eval("B& T F"), Value::Bool(false));
    assert_eq!(eval("B. S4% S34"), Value::Str(b"test".to_vec()));
    assert_eq!(eval("BT I$ S4%34"), Value::Str(b"tes".to_vec()));
    assert_eq!(eval("BD I$ S4%34"), Value::Str(b"t".to_vec()));
    assert_eq!(eval("? B> I# I$ S9%3 S./"), Value::Str(b"no".to_vec()));
}
