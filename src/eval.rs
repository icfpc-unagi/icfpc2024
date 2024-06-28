#![allow(non_snake_case)]

use itertools::Itertools;
use num_bigint::BigInt;

pub enum Node {
    Const(Value),
    Unary {
        op: u8,
        v: Box<Node>,
    },
    Binary {
        op: u8,
        v1: Box<Node>,
        v2: Box<Node>,
    },
    If {
        cond: Box<Node>,
        v1: Box<Node>,
        v2: Box<Node>,
    },
    Lambda {
        var: BigInt,
        exp: Box<Node>,
    },
}

fn parse(tokens: &[Vec<u8>], p: &mut usize) -> Node {
    let id = tokens[*p][0];
    let body = &tokens[*p][1..];
    *p += 1;
    match id {
        b'T' => {
            assert_eq!(body.len(), 0);
            Node::Const(Value::Bool(true))
        }
        b'F' => {
            assert_eq!(body.len(), 0);
            Node::Const(Value::Bool(false))
        }
        b'I' => {
            let mut val = BigInt::from(0);
            for &b in body {
                val *= 94;
                val += b - 33;
            }
            Node::Const(Value::Int(val))
        }
        b'S' => Node::Const(S(body)),
        b'U' => {
            assert_eq!(body.len(), 1);
            Node::Unary {
                op: body[0],
                v: Box::new(parse(tokens, p)),
            }
        }
        b'B' => {
            assert_eq!(body.len(), 1);
            Node::Binary {
                op: body[0],
                v1: Box::new(parse(tokens, p)),
                v2: Box::new(parse(tokens, p)),
            }
        }
        b'?' => {
            assert_eq!(body.len(), 0);
            Node::If {
                cond: Box::new(parse(tokens, p)),
                v1: Box::new(parse(tokens, p)),
                v2: Box::new(parse(tokens, p)),
            }
        }
        b'L' => {
            let mut var = BigInt::from(0);
            for &b in body {
                var *= 94;
                var += b - 33;
            }
            Node::Lambda {
                var,
                exp: Box::new(parse(tokens, p)),
            }
        }
        id => {
            panic!("unknown id: {}", id as char);
        }
    }
}

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

fn rec(root: Node, count: &mut usize) -> Node {
    match root {
        Node::Const(val) => Node::Const(val),
        Node::Unary { op, v } => match op {
            b'-' => {
                if let Node::Const(Value::Int(val)) = rec(*v, count) {
                    Node::Const(Value::Int(-val))
                } else {
                    panic!("negation of non-int");
                }
            }
            b'!' => {
                if let Node::Const(Value::Bool(val)) = rec(*v, count) {
                    Node::Const(Value::Bool(!val))
                } else {
                    panic!("negation of non-bool");
                }
            }
            b'#' => {
                if let Node::Const(Value::Str(val)) = rec(*v, count) {
                    let mut v = BigInt::from(0);
                    for b in val {
                        v *= 94;
                        v += CHARS.iter().position(|&c| c == b).unwrap();
                    }
                    Node::Const(Value::Int(v))
                } else {
                    panic!("length of non-str");
                }
            }
            b'$' => {
                if let Node::Const(Value::Int(mut val)) = rec(*v, count) {
                    let mut s = vec![];
                    while val > 0.into() {
                        let v: u8 = (val.clone() % BigInt::from(94)).try_into().unwrap();
                        s.push(v + 33);
                        val /= 94;
                    }
                    s.reverse();
                    Node::Const(S(&s))
                } else {
                    panic!("stringify of non-int");
                }
            }
            op => {
                panic!("unknown op: {}", op as char);
            }
        },
        Node::Binary { op, v1, v2 } => match op {
            b'+' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Int(a + b))
                    }
                    _ => panic!("addition of non-int"),
                }
            }
            b'-' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Int(a - b))
                    }
                    _ => panic!("subtraction of non-int"),
                }
            }
            b'*' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Int(a * b))
                    }
                    _ => panic!("multiplication of non-int"),
                }
            }
            b'/' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Int(a / b))
                    }
                    _ => panic!("division of non-int"),
                }
            }
            b'%' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Int(a % b))
                    }
                    _ => panic!("modulo of non-int"),
                }
            }
            b'<' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Bool(a < b))
                    }
                    _ => panic!("comparison of non-int"),
                }
            }
            b'>' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Bool(a > b))
                    }
                    _ => panic!("comparison of non-int"),
                }
            }
            b'=' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Node::Const(Value::Bool(a == b))
                    }
                    (Node::Const(Value::Bool(a)), Node::Const(Value::Bool(b))) => {
                        Node::Const(Value::Bool(a == b))
                    }
                    (Node::Const(Value::Str(a)), Node::Const(Value::Str(b))) => {
                        Node::Const(Value::Bool(a == b))
                    }
                    _ => panic!("comparison of different types"),
                }
            }
            b'|' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Bool(a)), Node::Const(Value::Bool(b))) => {
                        Node::Const(Value::Bool(a || b))
                    }
                    _ => panic!("or of non-bool"),
                }
            }
            b'&' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Bool(a)), Node::Const(Value::Bool(b))) => {
                        Node::Const(Value::Bool(a && b))
                    }
                    _ => panic!("or of non-bool"),
                }
            }
            b'.' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Str(mut a)), Node::Const(Value::Str(b))) => {
                        a.extend_from_slice(&b);
                        Node::Const(Value::Str(a))
                    }
                    _ => panic!("concat of non-str"),
                }
            }
            b'T' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Str(b))) => {
                        let a: usize = a.try_into().unwrap();
                        Node::Const(Value::Str(b[..a].to_vec()))
                    }
                    _ => panic!("take of non-int or non-str"),
                }
            }
            b'D' => {
                let a = rec(*v1, count);
                let b = rec(*v2, count);
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Str(b))) => {
                        let a: usize = a.try_into().unwrap();
                        Node::Const(Value::Str(b[a..].to_vec()))
                    }
                    _ => panic!("drop of non-int or non-str"),
                }
            }
            b'$' => {
                unimplemented!()
            }
            op => {
                panic!("unknown op: {}", op as char);
            }
        },
        Node::If { cond, v1, v2 } => {
            let cond = rec(*cond, count);
            match cond {
                Node::Const(Value::Bool(true)) => rec(*v1, count),
                Node::Const(Value::Bool(false)) => rec(*v2, count),
                _ => panic!("condition is not bool"),
            }
        }
        Node::Lambda { .. } => {
            panic!("lambda cannot be evaluated directly");
        }
    }
}

pub fn eval(s: &str) -> Value {
    let tokens = s
        .split_whitespace()
        .map(|s| s.bytes().collect_vec())
        .collect::<Vec<_>>();
    let mut p = 0;
    let root = parse(&tokens, &mut p);
    assert_eq!(p, tokens.len());
    let ret = rec(root, &mut 0);
    if let Node::Const(val) = ret {
        val
    } else {
        panic!("non-const result)");
    }
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
