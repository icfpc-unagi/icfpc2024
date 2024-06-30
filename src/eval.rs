#![allow(non_snake_case)]

use num::bigint::BigInt;
use std::result::Result::Ok;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Const(Value),
    Var(BigInt, Option<usize>), // name, de bruijn index
    Unary {
        op: u8,
        v: Rc<NodePos>,
    },
    Binary {
        op: u8,
        v1: Rc<NodePos>,
        v2: Rc<NodePos>,
    },
    If {
        cond: Rc<NodePos>,
        v1: Rc<NodePos>,
        v2: Rc<NodePos>,
    },
    Lambda {
        var: BigInt,
        exp: Rc<NodePos>,
    },
    Thunk(Rc<RefCell<NodePos>>),
}

#[derive(Clone, Debug)]
pub struct NodePos(pub Node, pub usize);

impl PartialEq for NodePos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn thunk(v: NodePos) -> Node {
    Node::Thunk(Rc::new(RefCell::new(v)))
}

fn to_icfp_string(val: &BigInt) -> String {
    let mut val = val.clone();
    let mut s = vec![];
    while val > 0.into() {
        let v: u8 = (val.clone() % BigInt::from(94)).try_into().unwrap();
        s.push(v + 33);
        val /= 94;
    }
    s.reverse();
    String::from_utf8_lossy(&s).to_string()
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::Const(val) => write!(f, "{}", val.pretty()),
            Node::Var(name, _index) => write!(f, "v{}", to_icfp_string(name)),
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
            Node::Lambda { var, exp } => {
                write!(f, "(\\v{} -> {})", to_icfp_string(var), exp)
            }
            // Return an fmt error.
            Node::Thunk(_) => write!(f, "ERROR: Thunk cannot be displayed"),
        }
    }
}

impl std::fmt::Display for NodePos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}:{}", self.0, self.1)
        } else {
            write!(f, "{}", self.0)
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
) -> anyhow::Result<NodePos> {
    if tokens.len() <= *p {
        anyhow::bail!("Token after {}: unexpected end of input", tokens[*p - 1].0);
    }
    let id = tokens[*p].1[0];
    let body = &tokens[*p].1[1..];
    *p += 1;
    match id {
        b'T' => match body.len() {
            0 => Ok(NodePos(Node::Const(Value::Bool(true)), tokens[*p - 1].0)),
            _ => anyhow::bail!(
                "Token at {}: T needs no argument, but: {}",
                tokens[*p - 1].0,
                body.iter().map(|&b| b as char).collect::<String>()
            ),
        },
        b'F' => match body.len() {
            0 => Ok(NodePos(Node::Const(Value::Bool(false)), tokens[*p - 1].0)),
            _ => anyhow::bail!(
                "Token at {}: F needs no argument, but: {}",
                tokens[*p - 1].0,
                body.iter().map(|&b| b as char).collect::<String>()
            ),
        },
        b'I' => {
            let mut val = BigInt::from(0);
            for (i, &b) in body.iter().enumerate() {
                match b {
                    b'!'..=b'~' => val = val * 94 + (b - 33),
                    _ => anyhow::bail!(
                        "Token at {}: invalid character in integer at {}",
                        tokens[*p - 1].0,
                        i
                    ),
                }
            }
            Ok(NodePos(Node::Const(Value::Int(val)), tokens[*p - 1].0))
        }
        b'S' => Ok(NodePos(Node::Const(S(body)), tokens[*p - 1].0)),
        b'U' => match body.len() {
            1 => Ok(NodePos(
                Node::Unary {
                    op: body[0],
                    v: Rc::new(parse(tokens, p, binders)?),
                },
                tokens[*p - 1].0,
            )),
            _ => anyhow::bail!(
                "Token at {}: U takes exactly one argument, but: {}",
                tokens[*p - 1].0,
                body.iter().map(|&b| b as char).collect::<String>()
            ),
        },
        b'B' => match body.len() {
            1 => Ok(NodePos(
                Node::Binary {
                    op: body[0],
                    v1: Rc::new(parse(tokens, p, binders)?),
                    v2: Rc::new(parse(tokens, p, binders)?),
                },
                tokens[*p - 1].0,
            )),
            _ => anyhow::bail!(
                "Token at {}: B takes exactly two arguments, but: {}",
                tokens[*p - 1].0,
                body.iter().map(|&b| b as char).collect::<String>()
            ),
        },
        b'?' => match body.len() {
            0 => Ok(NodePos(
                Node::If {
                    cond: Rc::new(parse(tokens, p, binders)?),
                    v1: Rc::new(parse(tokens, p, binders)?),
                    v2: Rc::new(parse(tokens, p, binders)?),
                },
                tokens[*p - 1].0,
            )),
            _ => anyhow::bail!(
                "Token at {}: ? takes no arguments, but: {}",
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
                        "Token at {}: invalid character '{}' in lambda argument at {}",
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
            Ok(NodePos(res, tokens[*p - 1].0))
        }
        b'v' => {
            let mut var = BigInt::from(0);
            for (i, &b) in body.iter().enumerate() {
                match b {
                    b'!'..=b'~' => var = var * 94 + (b - 33),
                    _ => anyhow::bail!(
                        "Token at {}: invalid character '{}' in variable at {}",
                        tokens[*p - 1].0,
                        b as char,
                        i
                    ),
                }
            }
            let de_bruijn_index = binders.iter().rev().position(|b| b == &var);
            Ok(NodePos(Node::Var(var, de_bruijn_index), tokens[*p - 1].0))
        }
        id => anyhow::bail!(
            "Token at {}: unknown token: {}",
            tokens[*p - 1].0,
            id as char
        ),
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

fn subst(
    root: &NodePos,
    var: &BigInt,
    val: Rc<NodePos>,
    level: usize,
) -> anyhow::Result<Rc<NodePos>> {
    let pos = root.1;
    match &root.0 {
        Node::Const(val) => Ok(Rc::new(NodePos(Node::Const(val.clone()), pos))),
        Node::Unary { op, v } => Ok(Rc::new(NodePos(
            Node::Unary {
                op: *op,
                v: subst(v, var, val, level)?,
            },
            pos,
        ))),
        Node::Binary { op, v1, v2 } => Ok(Rc::new(NodePos(
            Node::Binary {
                op: *op,
                v1: subst(v1, var, val.clone(), level)?,
                v2: subst(v2, var, val, level)?,
            },
            pos,
        ))),
        Node::If { cond, v1, v2 } => Ok(Rc::new(NodePos(
            Node::If {
                cond: subst(cond, var, val.clone(), level)?,
                v1: subst(v1, var, val.clone(), level)?,
                v2: subst(v2, var, val, level)?,
            },
            pos,
        ))),
        Node::Lambda { var: var2, exp } => Ok(Rc::new(NodePos(
            Node::Lambda {
                var: var2.clone(),
                exp: subst(exp, var, shift(val, level), level + 1)?,
            },
            pos,
        ))),
        Node::Var(var2, index) => {
            if index == &Some(level) {
                Ok(val.clone())
            } else {
                Ok(Rc::new(NodePos(Node::Var(var2.clone(), *index), pos)))
            }
        }
        Node::Thunk(_) => anyhow::bail!("Token {}: subst thunk", pos),
    }
}

fn shift(root: Rc<NodePos>, level: usize) -> Rc<NodePos> {
    let pos = root.1;
    match &root.0 {
        Node::Thunk(_) => todo!(),
        Node::Const(val) => Rc::new(NodePos(Node::Const(val.clone()), pos)),
        Node::Unary { op, v } => Rc::new(NodePos(
            Node::Unary {
                op: *op,
                v: shift(v.clone(), level),
            },
            pos,
        )),
        Node::Binary { op, v1, v2 } => Rc::new(NodePos(
            Node::Binary {
                op: *op,
                v1: shift(v1.clone(), level),
                v2: shift(v2.clone(), level),
            },
            pos,
        )),
        Node::If { cond, v1, v2 } => Rc::new(NodePos(
            Node::If {
                cond: shift(cond.clone(), level),
                v1: shift(v1.clone(), level),
                v2: shift(v2.clone(), level),
            },
            pos,
        )),
        Node::Lambda { var, exp } => Rc::new(NodePos {
            0: Node::Lambda {
                var: var.clone(),
                exp: shift(exp.clone(), level + 1),
            },
            1: pos,
        }),
        Node::Var(var, index) => {
            let index = index.map(|i| if i >= level { i + 1 } else { i });
            Rc::new(NodePos(Node::Var(var.clone(), index), pos))
        }
    }
}

fn rec(root: &NodePos, count: &mut usize) -> anyhow::Result<NodePos> {
    let pos = root.1;
    match &root.0 {
        Node::Const(val) => Ok(NodePos(Node::Const(val.clone()), pos)),
        Node::Unary { op, v } => match op {
            b'-' => match rec(&v, count)? {
                NodePos(Node::Const(Value::Int(val)), _) => {
                    Ok(NodePos(Node::Const(Value::Int(-val)), pos))
                }
                v => anyhow::bail!("Token {}: negation of non-int: {:#}", pos, v),
            },
            b'!' => match rec(&v, count)? {
                NodePos(Node::Const(Value::Bool(val)), _) => {
                    Ok(NodePos(Node::Const(Value::Bool(!val)), pos))
                }
                v => anyhow::bail!("Token {}: negation of non-bool: {:#}", pos, v),
            },
            b'#' => match rec(&v, count)? {
                NodePos(Node::Const(Value::Str(val)), _) => {
                    let mut v = BigInt::from(0);
                    for b in val {
                        v *= 94;
                        v += CHARS.iter().position(|&c| c == b).unwrap();
                    }
                    Ok(NodePos(Node::Const(Value::Int(v)), pos))
                }
                v => anyhow::bail!("Token {}: length of non-str: {:#}", pos, v),
            },
            b'$' => match rec(&v, count)? {
                NodePos(Node::Const(Value::Int(mut val)), _) => {
                    let mut s = vec![];
                    while val > 0.into() {
                        let v: u8 = (val.clone() % BigInt::from(94)).try_into().unwrap();
                        s.push(v + 33);
                        val /= 94;
                    }
                    s.reverse();
                    Ok(NodePos(Node::Const(S(&s)), pos))
                }
                v => anyhow::bail!("Token {}: stringify of non-int: {:#}", pos, v),
            },
            op => anyhow::bail!("Token {}: unknown unary op: {}", pos, *op as char),
        },
        Node::Binary { op, v1, v2 } => match op {
            b'+' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Int(a + b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: addition of non-int: {:#} + {:#}", pos, a, b)
                    }
                }
            }
            b'-' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Int(a - b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: subtraction of non-int: {:#} - {:#}", pos, a, b)
                    }
                }
            }
            b'*' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Int(a * b)), pos)),
                    (a, b) => anyhow::bail!(
                        "Token {}: multiplication of non-int: {:#} * {:#}",
                        pos,
                        a,
                        b
                    ),
                }
            }
            b'/' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Int(a / b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: division of non-int: {:#} / {:#}", pos, a, b)
                    }
                }
            }
            b'%' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Int(a % b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: modulo of non-int: {:#} % {:#}", pos, a, b)
                    }
                }
            }
            b'<' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Bool(a < b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: comparison of non-int: {:#} < {:#}", pos, a, b)
                    }
                }
            }
            b'>' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Bool(a > b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: comparison of non-int: {:#} > {:#}", pos, a, b)
                    }
                }
            }
            b'=' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Int(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Bool(a == b)), pos)),
                    (
                        NodePos(Node::Const(Value::Bool(a)), _),
                        NodePos(Node::Const(Value::Bool(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Bool(a == b)), pos)),
                    (
                        NodePos(Node::Const(Value::Str(a)), _),
                        NodePos(Node::Const(Value::Str(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Bool(a == b)), pos)),
                    (a, b) => anyhow::bail!(
                        "Token {}: comparison of different types: {:#} = {:#}",
                        pos,
                        a,
                        b
                    ),
                }
            }
            b'|' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Bool(a)), _),
                        NodePos(Node::Const(Value::Bool(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Bool(a || b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: or of non-bool: {:#} | {:#}", pos, a, b)
                    }
                }
            }
            b'&' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Bool(a)), _),
                        NodePos(Node::Const(Value::Bool(b)), _),
                    ) => Ok(NodePos(Node::Const(Value::Bool(a && b)), pos)),
                    (a, b) => {
                        anyhow::bail!("Token {}: and of non-bool: {:#} & {:#}", pos, a, b)
                    }
                }
            }
            b'.' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Str(mut a)), _),
                        NodePos(Node::Const(Value::Str(b)), _),
                    ) => {
                        a.extend_from_slice(&b);
                        Ok(NodePos(Node::Const(Value::Str(a)), pos))
                    }
                    (a, b) => {
                        anyhow::bail!("Token {}: concat of non-str: {:#} . {:#}", pos, a, b)
                    }
                }
            }
            b'T' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Str(b)), _),
                    ) => {
                        let a: usize = a.try_into().unwrap();
                        Ok(NodePos(Node::Const(Value::Str(b[..a].to_vec())), pos))
                    }
                    (a, b) => anyhow::bail!(
                        "Token {}: take of non-int or non-str: {:#} T {:#}",
                        pos,
                        a,
                        b
                    ),
                }
            }
            b'D' => {
                let a = rec(&v1, count)?;
                let b = rec(&v2, count)?;
                match (a, b) {
                    (
                        NodePos(Node::Const(Value::Int(a)), _),
                        NodePos(Node::Const(Value::Str(b)), _),
                    ) => {
                        let a: usize = a.try_into().unwrap();
                        Ok(NodePos(Node::Const(Value::Str(b[a..].to_vec())), pos))
                    }
                    (a, b) => anyhow::bail!(
                        "Token {}: drop of non-int or non-str: {:#} D {:#}",
                        pos,
                        a,
                        b
                    ),
                }
            }
            b'$' => {
                let lambda = rec(&v1, count)?;
                match lambda {
                    NodePos(Node::Lambda { var, exp }, _) => {
                        *count += 1;
                        if *count > 10_000_000 {
                            anyhow::bail!(
                                "Token {}: beta reductions limit exceeded: {:#}",
                                pos,
                                exp
                            );
                        }
                        let v = subst(&exp, &var, v2.clone(), 0)?;
                        rec(&v, count)
                    }
                    v => anyhow::bail!("Token {}: apply of non-lambda: {:#}", pos, v),
                }
            }
            b'!' => {
                // call by value
                let lambda = rec(&v1, count)?;
                match lambda {
                    NodePos(Node::Lambda { var, exp }, _) => {
                        let b = rec(&v2, count)?;
                        *count += 1;
                        if *count > 10_000_000 {
                            anyhow::bail!(
                                "Token {}: beta reductions limit exceeded: {:#}",
                                pos,
                                exp
                            );
                        }
                        let v = subst(&exp, &var, Rc::new(b), 0)?;
                        rec(&v, count)
                    }
                    v => anyhow::bail!("Token {}: apply of non-lambda: {:#}", pos, v),
                }
            }
            b'~' => {
                // call-by-need
                let lambda = rec(&v1, count)?;
                match lambda {
                    NodePos(Node::Lambda { var, exp }, _) => {
                        *count += 1;
                        if *count > 10_000_000 {
                            anyhow::bail!(
                                "Token {}: beta reductions limit exceeded: {:#}",
                                pos,
                                exp
                            );
                        }
                        let v2: NodePos = (**v2).clone();
                        let v2 = NodePos(thunk(v2), pos);
                        let v = subst(&exp, &var, Rc::new(v2), 0)?;
                        rec(&v, count)
                    }
                    v => anyhow::bail!("Token {}: apply of non-lambda: {:#}", pos, v),
                }
            }
            op => anyhow::bail!("Token {}: unknown binary op: {}", pos, *op as char),
        },
        Node::If { cond, v1, v2 } => {
            let cond = rec(&cond, count)?;
            match cond {
                NodePos(Node::Const(Value::Bool(true)), _) => rec(&v1, count),
                NodePos(Node::Const(Value::Bool(false)), _) => rec(&v2, count),
                v => anyhow::bail!("Token {}: condition is not bool: {:#}", pos, v),
            }
        }
        Node::Lambda { var, exp } => Ok(NodePos(
            Node::Lambda {
                var: var.clone(),
                exp: exp.clone(),
            },
            pos,
        )),
        Node::Var(var, index) => Ok(NodePos(Node::Var(var.clone(), *index), pos)),
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
        if ch.is_whitespace() != in_whitespace {
            if ch.is_whitespace() {
                tokens.push((start, input[start..index].bytes().collect()));
            }
            start = index;
            in_whitespace = ch.is_whitespace();
        }
    }

    if !in_whitespace {
        tokens.push((start, input[start..].bytes().collect()));
    }

    tokens
}

fn eval_to_node(s: &str) -> anyhow::Result<NodePos> {
    let tokens = tokenize(s);
    let mut p = 0;
    let mut binders = vec![];
    let root = parse(&tokens, &mut p, &mut binders)?;
    assert_eq!(p, tokens.len());
    assert_eq!(binders.len(), 0);
    rec(&root, &mut 0)
}

pub fn eval(s: &str) -> anyhow::Result<Value> {
    match eval_to_node(s)? {
        NodePos(Node::Const(val), _) => Ok(val),
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

    assert_eq!(
        eval_to_node("B$ Lx vy I0").unwrap(),
        eval_to_node("vy").unwrap()
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

#[test]
fn test_position() {
    // 値はそのポジションをとるべき
    assert_eq!(eval_to_node("I0").unwrap().1, 0);
    // 計算結果は計算結果の位置になって欲しい
    // assert_eq!(eval_to_node("B+ I0 I0").unwrap().1, 0);
}

#[test]
fn test_errors() {
    // 一項演算子のあとがないパターン
    assert_eq!(
        format!("{}", eval_to_node("U-").unwrap_err()),
        "Token after 0: unexpected end of input"
    );
    // 二項演算子のあとがないパターン
    assert_eq!(
        format!("{}", eval_to_node("B. B.").unwrap_err()),
        "Token after 3: unexpected end of input"
    );
    // 文字列演算ができないパターン
    assert_eq!(
        format!("{}", eval_to_node("B. I0 S").unwrap_err()),
        r#"Token 6: concat of non-str: 15:3 . "":6"#,
    );
    // 整数演算ができないパターン
    assert_eq!(
        format!("{}", eval_to_node("B+ I0 S").unwrap_err()),
        r#"Token 6: addition of non-int: 15:3 + "":6"#,
    );
    // 計算結果の演算ができないパターン
    // assert_eq!(
    //     format!("{}", eval_to_node("B+ I0 B. S0 S1").unwrap_err()),
    //     r#"Token 12: addition of non-int: 15:3 + "pq":6"#,
    // );
}
