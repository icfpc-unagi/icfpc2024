#![allow(non_snake_case)]

// use anyhow::Ok;
use itertools::Itertools;
use num::bigint::BigInt;
use std::result::Result::Ok;
use std::{cell::RefCell, io::Read, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Const(Value),
    Var(BigInt, Option<usize>), // name, de bruijn index
    Unary {
        op: u8,
        v: Rc<Node>,
    },
    Binary {
        op: u8,
        v1: Rc<Node>,
        v2: Rc<Node>,
    },
    If {
        cond: Rc<Node>,
        v1: Rc<Node>,
        v2: Rc<Node>,
    },
    Lambda {
        var: BigInt,
        exp: Rc<Node>,
    },
    Thunk(Rc<RefCell<Node>>),
}

fn thunk(v: Node) -> Node {
    Node::Thunk(Rc::new(RefCell::new(v)))
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::Const(val) => write!(f, "{}", val.pretty()),
            Node::Var(name, _index) => {
                // we don't need index for input (before reduction)
                write!(f, "v{}", name)
                // if let Some(index) = index {
                //     write!(f, "v{}({})", name, index)
                // } else {
                //     write!(f, "v{}", name)
                // }
            }
            Node::Unary { op, v } => write!(f, "({} {})", *op as char, v),
            Node::Binary { op, v1, v2 } => {
                let op = *op as char;
                if let Some(fullname) = match op {
                    'T' => Some("take"),
                    'D' => Some("drop"),
                    _ => None,
                } {
                    return write!(f, "({} {} {})", fullname, v1, v2);
                } else {
                    write!(f, "({} {} {})", v1, op, v2)
                }
            }
            Node::If { cond, v1, v2 } => write!(f, "({} ? {} : {})", cond, v1, v2),
            Node::Lambda { var, exp } => write!(f, "(\\v{} -> {})", var, exp),
            Node::Thunk(_) => panic!("display thunk"),
        }
    }
}

pub fn debug_parse(s: &str) -> () {
    let tokens = tokenize(s);
    let mut p = 0;
    let mut binders = vec![];
    let res = parse(&tokens, &mut p, &mut binders);
    assert_eq!(p, tokens.len());
    match res {
        Ok(node) => eprintln!("{}", node),
        Err(e) => eprintln!("Error: {}", e),
    };
}

pub fn parse(
    tokens: &[(usize, Vec<u8>)],
    p: &mut usize,
    binders: &mut Vec<BigInt>,
) -> anyhow::Result<Node> {
    let id = tokens[*p].1[0];
    let body = &tokens[*p].1[1..];
    *p += 1;
    match id {
        b'T' => match body.len() {
            0 => Ok(Node::Const(Value::Bool(true))),
            _ => anyhow::bail!("{}: T needs an argument", tokens[*p - 1].0),
        },
        b'F' => match body.len() {
            0 => Ok(Node::Const(Value::Bool(false))),
            _ => anyhow::bail!("{}: F needs an argument", tokens[*p - 1].0),
        },
        b'I' => {
            let mut val = BigInt::from(0);
            for (i, &b) in body.iter().enumerate() {
                match b {
                    b'!'..=b'~' => val = val * 94 + (b - 33),
                    _ => anyhow::bail!(
                        "{}: invalid character in integer at {}",
                        tokens[*p - 1].0,
                        i
                    ),
                }
            }
            Ok(Node::Const(Value::Int(val)))
        }
        b'S' => Ok(Node::Const(S(body))),
        b'U' => match body.len() {
            1 => Ok(Node::Unary {
                op: body[0],
                v: Rc::new(parse(tokens, p, binders)?),
            }),
            _ => anyhow::bail!(
                "{}: U takes exactly one argument, but: {}",
                tokens[*p - 1].0,
                body.iter().map(|&b| b as char).collect::<String>()
            ),
        },
        b'B' => match body.len() {
            1 => Ok(Node::Binary {
                op: body[0],
                v1: Rc::new(parse(tokens, p, binders)?),
                v2: Rc::new(parse(tokens, p, binders)?),
            }),
            _ => anyhow::bail!(
                "{}: B takes exactly two arguments, but: {}",
                tokens[*p - 1].0,
                body.iter().map(|&b| b as char).collect::<String>()
            ),
        },
        b'?' => match body.len() {
            0 => Ok(Node::If {
                cond: Rc::new(parse(tokens, p, binders)?),
                v1: Rc::new(parse(tokens, p, binders)?),
                v2: Rc::new(parse(tokens, p, binders)?),
            }),
            _ => anyhow::bail!(
                "{}: ? takes exactly three arguments, but: {}",
                tokens[*p - 1].0,
                body.iter().map(|&b| b as char).collect::<String>()
            ),
        },
        b'L' => {
            let mut var = BigInt::from(0);
            for (i, &b) in body.iter().enumerate() {
                match b {
                    b'!'..=b'~' => var = var * 94 + (b - 33),
                    _ => anyhow::bail!(
                        "{}: invalid character '{}' in lambda argument at {}",
                        tokens[*p - 1].0,
                        b as char,
                        i
                    ),
                }
            }
            binders.push(var.clone());
            let res = Node::Lambda {
                var,
                exp: Rc::new(parse(tokens, p, binders)?),
            };
            binders.pop();
            Ok(res)
        }
        b'v' => {
            let mut var = BigInt::from(0);
            for (i, &b) in body.iter().enumerate() {
                match b {
                    b'!'..=b'~' => var = var * 94 + (b - 33),
                    _ => anyhow::bail!(
                        "{}: invalid character '{}' in variable at {}",
                        tokens[*p - 1].0,
                        b as char,
                        i
                    ),
                }
            }
            let de_bruijn_index = binders.iter().rev().position(|b| b == &var);
            Ok(Node::Var(var, de_bruijn_index))
        }
        id => anyhow::bail!("{}: unknown token: {}", tokens[*p - 1].0, id as char),
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

impl Value {
    fn pretty(&self) -> String {
        match self {
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Str(s) => format!("{:?}", String::from_utf8_lossy(s)),
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

fn subst(root: &Node, var: &BigInt, val: Rc<Node>, level: usize) -> Rc<Node> {
    match root {
        Node::Const(val) => Rc::new(Node::Const(val.clone())),
        Node::Unary { op, v } => Rc::new(Node::Unary {
            op: *op,
            v: subst(v, var, val, level),
        }),
        Node::Binary { op, v1, v2 } => Rc::new(Node::Binary {
            op: *op,
            v1: subst(v1, var, val.clone(), level),
            v2: subst(v2, var, val, level),
        }),
        Node::If { cond, v1, v2 } => Rc::new(Node::If {
            cond: subst(cond, var, val.clone(), level),
            v1: subst(v1, var, val.clone(), level),
            v2: subst(v2, var, val, level),
        }),
        Node::Lambda { var: var2, exp } => Rc::new(Node::Lambda {
            var: var2.clone(),
            // TODO: cache shifted val
            exp: subst(exp, var, shift(val, level), level + 1),
        }),
        Node::Var(var2, index) => {
            if index == &Some(level) {
                val.clone()
            } else {
                Rc::new(Node::Var(var2.clone(), *index))
            }
        }
        Node::Thunk(_) => panic!("unevaluated thunk"),
    }
}

fn shift(root: Rc<Node>, level: usize) -> Rc<Node> {
    match root.as_ref() {
        Node::Thunk(_) => todo!(),
        Node::Const(val) => Rc::new(Node::Const(val.clone())),
        Node::Unary { op, v } => Rc::new(Node::Unary {
            op: *op,
            v: shift(v.clone(), level),
        }),
        Node::Binary { op, v1, v2 } => Rc::new(Node::Binary {
            op: *op,
            v1: shift(v1.clone(), level),
            v2: shift(v2.clone(), level),
        }),
        Node::If { cond, v1, v2 } => Rc::new(Node::If {
            cond: shift(cond.clone(), level),
            v1: shift(v1.clone(), level),
            v2: shift(v2.clone(), level),
        }),
        Node::Lambda { var, exp } => Rc::new(Node::Lambda {
            var: var.clone(),
            exp: shift(exp.clone(), level + 1),
        }),
        Node::Var(var, index) => {
            let index = index.map(|i| if i >= level { i + 1 } else { i });
            Rc::new(Node::Var(var.clone(), index))
        }
    }
}

fn rec(root: &Node, count: &mut usize) -> anyhow::Result<Node> {
    match root {
        Node::Const(val) => Ok(Node::Const(val.clone())),
        Node::Unary { op, v } => match op {
            b'-' => {
                if let Node::Const(Value::Int(val)) = rec(v, count)? {
                    Ok(Node::Const(Value::Int(-val)))
                } else {
                    panic!("negation of non-int");
                }
            }
            b'!' => {
                if let Node::Const(Value::Bool(val)) = rec(v, count)? {
                    Ok(Node::Const(Value::Bool(!val)))
                } else {
                    panic!("negation of non-bool");
                }
            }
            b'#' => {
                if let Node::Const(Value::Str(val)) = rec(v, count)? {
                    let mut v = BigInt::from(0);
                    for b in val {
                        v *= 94;
                        v += CHARS.iter().position(|&c| c == b).unwrap();
                    }
                    Ok(Node::Const(Value::Int(v)))
                } else {
                    panic!("length of non-str");
                }
            }
            b'$' => {
                if let Node::Const(Value::Int(mut val)) = rec(v, count)? {
                    let mut s = vec![];
                    while val > 0.into() {
                        let v: u8 = (val.clone() % BigInt::from(94)).try_into().unwrap();
                        s.push(v + 33);
                        val /= 94;
                    }
                    s.reverse();
                    Ok(Node::Const(S(&s)))
                } else {
                    panic!("stringify of non-int");
                }
            }
            op => {
                panic!("unknown op: {}", *op as char);
            }
        },
        Node::Binary { op, v1, v2 } => match *op {
            b'+' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Int(a + b)))
                    }
                    _ => panic!("addition of non-int"),
                }
            }
            b'-' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Int(a - b)))
                    }
                    _ => panic!("subtraction of non-int"),
                }
            }
            b'*' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Int(a * b)))
                    }
                    _ => panic!("multiplication of non-int"),
                }
            }
            b'/' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Int(a / b)))
                    }
                    _ => panic!("division of non-int"),
                }
            }
            b'%' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Int(a % b)))
                    }
                    _ => panic!("modulo of non-int"),
                }
            }
            b'<' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Bool(a < b)))
                    }
                    _ => panic!("comparison of non-int"),
                }
            }
            b'>' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Bool(a > b)))
                    }
                    _ => panic!("comparison of non-int"),
                }
            }
            b'=' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Int(b))) => {
                        Ok(Node::Const(Value::Bool(a == b)))
                    }
                    (Node::Const(Value::Bool(a)), Node::Const(Value::Bool(b))) => {
                        Ok(Node::Const(Value::Bool(a == b)))
                    }
                    (Node::Const(Value::Str(a)), Node::Const(Value::Str(b))) => {
                        Ok(Node::Const(Value::Bool(a == b)))
                    }
                    _ => panic!("comparison of different types"),
                }
            }
            b'|' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Bool(a)), Node::Const(Value::Bool(b))) => {
                        Ok(Node::Const(Value::Bool(a || b)))
                    }
                    _ => panic!("or of non-bool"),
                }
            }
            b'&' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Bool(a)), Node::Const(Value::Bool(b))) => {
                        Ok(Node::Const(Value::Bool(a && b)))
                    }
                    _ => panic!("or of non-bool"),
                }
            }
            b'.' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Str(mut a)), Node::Const(Value::Str(b))) => {
                        a.extend_from_slice(&b);
                        Ok(Node::Const(Value::Str(a)))
                    }
                    _ => panic!("concat of non-str"),
                }
            }
            b'T' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Str(b))) => {
                        let a: usize = a.try_into().unwrap();
                        Ok(Node::Const(Value::Str(b[..a].to_vec())))
                    }
                    _ => panic!("take of non-int or non-str"),
                }
            }
            b'D' => {
                let a = rec(v1, count)?;
                let b = rec(v2, count)?;
                match (a, b) {
                    (Node::Const(Value::Int(a)), Node::Const(Value::Str(b))) => {
                        let a: usize = a.try_into().unwrap();
                        Ok(Node::Const(Value::Str(b[a..].to_vec())))
                    }
                    _ => panic!("drop of non-int or non-str"),
                }
            }
            b'$' => {
                let lambda = rec(v1, count)?;
                if let Node::Lambda { var, exp } = lambda {
                    *count += 1;
                    if *count > 10_000_000 {
                        panic!("beta reductions limit exceeded");
                    }
                    // dbg!(&exp, &var, &v2);
                    let v = subst(&exp, &var, v2.clone(), 0);
                    // dbg!(&v);
                    rec(&v, count)
                } else {
                    panic!("apply of non-lambda");
                }
            }
            b'!' => {
                // call by value
                let lambda = rec(v1, count)?;
                if let Node::Lambda { var, exp } = lambda {
                    // dbg!(&v2);
                    let b = rec(v2, count)?;
                    // dbg!(&b);
                    *count += 1;
                    if *count > 10_000_000 {
                        panic!("beta reductions limit exceeded");
                    }
                    // dbg!(&exp, &var);
                    let v = subst(&exp, &var, Rc::new(b), 0);
                    // dbg!(v.clone());
                    rec(v.as_ref(), count)
                } else {
                    panic!("apply of non-lambda");
                }
            }
            b'~' => {
                // call-by-need
                let lambda = rec(v1, count)?;
                if let Node::Lambda { var, exp } = lambda {
                    *count += 1;
                    if *count > 10_000_000 {
                        panic!("beta reductions limit exceeded");
                    }
                    let v2: Node = (**v2).clone();
                    let v2 = thunk(v2);
                    let v = subst(&exp, &var, Rc::new(v2), 0);
                    rec(&v, count)
                } else {
                    panic!("apply of non-lambda");
                }
            }
            op => {
                panic!("unknown op: {}", op as char);
            }
        },
        Node::If { cond, v1, v2 } => {
            let cond = rec(cond, count)?;
            match cond {
                Node::Const(Value::Bool(true)) => rec(v1, count),
                Node::Const(Value::Bool(false)) => rec(v2, count),
                _ => panic!("condition is not bool"),
            }
        }
        Node::Lambda { var, exp } => Ok(Node::Lambda {
            var: var.clone(),
            // do not evaluate exp here
            exp: exp.clone(),
        }),
        Node::Var(var, index) => Ok(Node::Var(var.clone(), *index)),
        Node::Thunk(_v) => {
            // let v_inner = v.get_mut();
            todo!()
        }
    }
}

fn tokenize(input: &str) -> Vec<(usize, Vec<u8>)> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut in_whitespace = false;

    for (index, ch) in input.char_indices() {
        if ch.is_whitespace() {
            if !in_whitespace {
                // 非空白文字列のトークンを追加
                if start != index {
                    tokens.push((start, input[start..index].bytes().collect()));
                }
                start = index;
                in_whitespace = true;
            }
        } else {
            if in_whitespace {
                start = index;
                in_whitespace = false;
            }
        }
    }

    // 最後のトークンを追加
    if !in_whitespace && start < input.len() {
        tokens.push((start, input[start..].bytes().collect()));
    }

    tokens
}

fn eval_to_node(s: &str) -> anyhow::Result<Node> {
    let tokens = tokenize(s);
    let mut p = 0;
    let mut binders = vec![];
    let root = parse(&tokens, &mut p, &mut binders)?;
    assert_eq!(p, tokens.len());
    assert_eq!(binders.len(), 0);
    rec(&root, &mut 0)
}

// fn eval_to_node(s: &str) -> Node {
//     let tokens: Vec<Vec<u8>> = s
//         .split_whitespace()
//         .map(|s| s.bytes().collect_vec())
//         .collect::<Vec<_>>();
//     let mut p = 0;
//     let mut binders = vec![];
//     let root = parse(&tokens, &mut p, &mut binders);
//     assert_eq!(p, tokens.len());
//     assert_eq!(binders.len(), 0);
//     rec(&root, &mut 0)
// }

pub fn eval(s: &str) -> anyhow::Result<Value> {
    match eval_to_node(s)? {
        Node::Const(val) => Ok(val),
        v => anyhow::bail!("Non-const result: {}", v),
    }
}

#[test]
fn test() {
    assert_eq!(eval("I/6").unwrap(), Value::Int(1337.into()));
    assert_eq!(
        eval("SB%,,/}Q/2,$_").unwrap(),
        Value::Str(b"Hello World!".to_vec())
    );
    assert_eq!(eval("U- I$").unwrap(), Value::Int((-3).into()));
    assert_eq!(eval("U! T").unwrap(), Value::Bool(false));
    assert_eq!(eval("U# S4%34").unwrap(), Value::Int(15818151.into()));
    assert_eq!(eval("U$ I4%34").unwrap(), Value::Str(b"test".to_vec()));
    assert_eq!(eval("B+ I# I$").unwrap(), Value::Int(5.into()));
    assert_eq!(eval("B- I$ I#").unwrap(), Value::Int(1.into()));
    assert_eq!(eval("B* I$ I#").unwrap(), Value::Int(6.into()));
    assert_eq!(eval("B/ U- I( I#").unwrap(), Value::Int((-3).into()));
    assert_eq!(eval("B% U- I( I#").unwrap(), Value::Int((-1).into()));
    assert_eq!(eval("B< I$ I#").unwrap(), Value::Bool(false));
    assert_eq!(eval("B> I$ I#").unwrap(), Value::Bool(true));
    assert_eq!(eval("B= I$ I#").unwrap(), Value::Bool(false));
    assert_eq!(eval("B| T F").unwrap(), Value::Bool(true));
    assert_eq!(eval("B& T F").unwrap(), Value::Bool(false));
    assert_eq!(eval("B. S4% S34").unwrap(), Value::Str(b"test".to_vec()));
    assert_eq!(eval("BT I$ S4%34").unwrap(), Value::Str(b"tes".to_vec()));
    assert_eq!(eval("BD I$ S4%34").unwrap(), Value::Str(b"t".to_vec()));
    assert_eq!(
        eval("? B> I# I$ S9%3 S./").unwrap(),
        Value::Str(b"no".to_vec())
    );
    assert_eq!(
        eval("B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK").unwrap(),
        Value::Str(b"Hello World!".to_vec())
    );
    assert_eq!(
        eval(r#"B$ L# B$ L" B+ v" v" B* I$ I# v8"#).unwrap(),
        eval("I-").unwrap()
    );
}

#[test]
fn test2() {
    // (\y. (\x. (\y. x + y)) y) 3 4
    assert_eq!(
        eval("B$ B$ B$ Ly Lx Ly B+ vx vy vy I$ I%").unwrap(),
        Value::Int(7.into())
    );

    // (\x. x + x) 3
    assert_eq!(eval("B$ Lx B+ vx vx I$").unwrap(), Value::Int(6.into()));
    // (\x. (\f. f (f (f x))) (\x. x + x)) 3
    assert_eq!(
        eval("B$ Lx B$ Lf B$ vf B$ vf B$ vf vx Lx B+ vx vx I$").unwrap(),
        Value::Int(24.into())
    );
    // (\y. (\x. (\f. f (f (f x))) (\x. x * y)) 3) 4
    assert_eq!(
        eval("B$ Ly B$ Lx B$ Lf B$ vf B$ vf B$ vf vx Lx B* vx vy I$ I%").unwrap(),
        Value::Int(192.into())
    );
    // (\x. (\y. (\f. f (f (f x))) (\x. x * y)) 4) 3
    assert_eq!(
        eval("B$ Lx B$ Ly B$ Lf B$ vf B$ vf B$ vf vx Lx B* vx vy I% I$").unwrap(),
        Value::Int(192.into())
    );

    // 5! == 120
    // f(x) := If x > 0 then x * f(x - 1) else 1

    // {\displaystyle Y=\lambda f.\ (\lambda x.f\ (x\ x))\ (\lambda x.f\ (x\ x))}
    // y = \f. (\x. f (x x)) (\x. f (x x))
    let y = "Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx";
    let f = format!(r#"B$ {y} Lf Lx ? B> vx I! B* vx B$ vf B- vx I" I""#);
    assert_eq!(eval(&format!("B$ {f} I&")).unwrap(), Value::Int(120.into()));
}

#[test]
fn test_reduction() {
    // (\x. x) [x := y]
    // \x. x
    assert_eq!(
        eval_to_node("B$ Lx Lx vx vy").unwrap(),
        eval_to_node("Lx vx").unwrap()
    );

    // // (\y. x + y) [x := y]
    // // (\z. y + z)
    // let mut expected = eval_to_node("Lz B+ vy vz")
    // // equal up to variable names (we use de bruijn indices)
    // assert_eq!(eval_to_node("B$ Lx Ly B+ vx vy vy"), expected);

    // (\f. \x. \y. f x y) (\a. \b. a + b - z)
    // \x. \y. (\a. \b. a + b - z) x y
    // // \x. \y. x + y - z
    assert_eq!(
        eval_to_node("B$ Lf Lx Ly B$ B$ vf vx vy La Lb B- B+ va vb vz").unwrap(),
        eval_to_node("Lx Ly B$ B$ La Lb B- B+ va vb vz vx vy").unwrap()
    );

    // (1-origin)
    // (λ λ 4 2 (λ 1 3)) (λ 5 1)
    // (λx. λy. z x (λu. u x)) (λx. w x)
    // -> λ 3 (λ 6 1) (λ 1 (λ 7 1))
    // λy. z (λx. w x) (λu. u (λx. w x))
    let m = "Lx Ly B$ B$ vz vx Lu B$ vu vx";
    let n = "Lx B$ vw vx";
    let reduced = eval_to_node(&format!("B$ B$ B$ B$ Lw Lp Lz Lq B$ {m} {n} Sw Sp Sz Sq")).unwrap();
    let expected = eval_to_node("Ly B$ B$ Sz Lx B$ Sw vx Lu B$ vu Lx B$ Sw vx").unwrap();
    eprintln!("{}", reduced);
    eprintln!("{}", expected);
    dbg!(&reduced, &expected);
    assert_eq!(reduced, expected);
}
