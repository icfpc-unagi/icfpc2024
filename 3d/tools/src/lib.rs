#![allow(non_snake_case, unused_macros)]

use num::*;
use num_bigint::BigInt;
use std::convert::TryInto;
use svg::node::element::{Group, Line, Rectangle, Style, Title};

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
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
const DIR: [char; 4] = ['v', '>', '^', '<'];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum P {
    Empty,
    Num(BigInt),
    Move(usize),
    Op(char),
    Warp,
    Eq,
    Neq,
    Submit,
    Input(usize),
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            P::Empty => write!(f, "."),
            P::Num(n) => write!(f, "{}", n),
            P::Move(d) => write!(f, "{}", DIR[*d]),
            P::Op(c) => write!(f, "{}", c),
            P::Warp => write!(f, "@"),
            P::Eq => write!(f, "="),
            P::Neq => write!(f, "#"),
            P::Submit => write!(f, "S"),
            P::Input(i) => write!(f, "{}", (b'A' + *i as u8) as char),
        }
    }
}

fn parse_P(s: &str) -> Result<P, String> {
    match s {
        "." => Ok(P::Empty),
        "@" => Ok(P::Warp),
        "=" => Ok(P::Eq),
        "#" => Ok(P::Neq),
        "S" => Ok(P::Submit),
        "+" | "-" | "*" | "/" | "%" => Ok(P::Op(s.chars().next().unwrap())),
        "A" => Ok(P::Input(0)),
        "B" => Ok(P::Input(1)),
        _ => {
            if s.len() == 1 {
                if let Some(d) = DIR.iter().position(|&c| c == s.chars().next().unwrap()) {
                    return Ok(P::Move(d));
                }
            }
            if let Ok(num) = s.parse::<BigInt>() {
                return Ok(P::Num(num));
            }
            return Err(format!("Unknown op: {}", s));
        }
    }
}

pub struct Output {
    pub init: Vec<Vec<P>>,
}

pub fn parse_output(f: &str) -> Result<Output, String> {
    let mut init = vec![];
    for line in f.lines().skip(1) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let ss = line.split_whitespace().collect::<Vec<_>>();
        let mut row = vec![];
        for s in ss {
            row.push(parse_P(s)?);
        }
        init.push(row);
    }
    Ok(Output { init })
}

pub struct Sim {
    pub score: i64,
    pub ret: P,
    pub err: String,
    pub log: Vec<(usize, Vec<Vec<P>>)>,
}

pub fn compute_score(out: &Output, input: &[BigInt]) -> Sim {
    let n = out.init.len();
    let m = out.init[0].len();
    let mut log = vec![];
    let mut crt = out.init.clone();
    for i in 0..n {
        for j in 0..m {
            if let P::Input(id) = crt[i][j] {
                crt[i][j] = P::Num(input[id].clone());
            }
        }
    }
    let mut t = 0;
    let mut times = vec![crt.clone()];
    while log.len() < 10000 {
        log.push((t, crt.clone()));
        if t >= times.len() {
            times.push(crt.clone());
        } else {
            times[t] = crt.clone();
        }
        let mut next = crt.clone();
        let mut next_t = !0;
        for i in 0..n {
            for j in 0..m {
                if crt[i][j] == P::Warp {
                    if i > 0 && crt[i - 1][j] != P::Empty {
                        if j > 0 && matches!(crt[i][j - 1], P::Num(_)) {
                            if i + 1 < n && matches!(crt[i + 1][j], P::Num(_)) {
                                if j + 1 < m && matches!(crt[i][j + 1], P::Num(_)) {
                                    let P::Num(ref dt) = crt[i + 1][j] else {
                                        unreachable!()
                                    };
                                    if dt <= &BigInt::from(0) || dt > &BigInt::from(t) {
                                        return Sim {
                                            score: 0,
                                            ret: P::Empty,
                                            err: format!("Invalid dt: {}", dt),
                                            log,
                                        };
                                    }
                                    let dt: usize = dt.try_into().unwrap();
                                    if next_t != !0 && next_t != t - dt {
                                        return Sim {
                                            score: 0,
                                            ret: P::Empty,
                                            err: format!("Conflict dt: {} vs {}", t - next_t, dt),
                                            log,
                                        };
                                    }
                                    next_t = t - dt;
                                }
                            }
                        }
                    }
                }
            }
        }
        if next_t != !0 {
            let mut write = mat![P::Empty; n; m];
            for i in 0..n {
                for j in 0..m {
                    if crt[i][j] == P::Warp {
                        if i > 0 && crt[i - 1][j] != P::Empty {
                            if j > 0 && matches!(crt[i][j - 1], P::Num(_)) {
                                if i + 1 < n && matches!(crt[i + 1][j], P::Num(_)) {
                                    if j + 1 < m && matches!(crt[i][j + 1], P::Num(_)) {
                                        let P::Num(ref dx) = crt[i][j - 1] else {
                                            unreachable!()
                                        };
                                        let P::Num(ref dy) = crt[i][j + 1] else {
                                            unreachable!()
                                        };
                                        if -dx + j < BigInt::from(0) || -dx + j >= BigInt::from(m) {
                                            return Sim {
                                                score: 0,
                                                ret: P::Empty,
                                                err: format!("Invalid dx: {}", dx),
                                                log,
                                            };
                                        }
                                        if -dy + i < BigInt::from(0) || -dy + i >= BigInt::from(n) {
                                            return Sim {
                                                score: 0,
                                                ret: P::Empty,
                                                err: format!("Invalid dy: {}", dy),
                                                log,
                                            };
                                        }
                                        let dx: i32 = dx.try_into().unwrap();
                                        let dy: i32 = dy.try_into().unwrap();
                                        let dx = dx as usize;
                                        let dy = dy as usize;
                                        if write[i - dy][j - dx] != P::Empty
                                            && write[i - dy][j - dx] != crt[i - 1][j]
                                        {
                                            return Sim {
                                                score: 0,
                                                ret: P::Empty,
                                                err: format!(
                                                    "Conflict write: {} {}",
                                                    i - dy,
                                                    j - dx
                                                ),
                                                log,
                                            };
                                        }
                                        write[i - dy][j - dx] = crt[i - 1][j].clone();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            next = times[next_t].clone();
            for i in 0..n {
                for j in 0..m {
                    if write[i][j] != P::Empty {
                        if next[i][j] == P::Submit {
                            return Sim {
                                score: (n * m * (times.len() + 1)) as i64,
                                ret: write[i][j].clone(),
                                err: String::new(),
                                log,
                            };
                        }
                        next[i][j] = write[i][j].clone();
                    }
                }
            }
        } else {
            let mut read = mat![false; n; m];
            let mut write = mat![P::Empty; n; m];
            for i in 0..n {
                for j in 0..m {
                    match crt[i][j] {
                        P::Move(dir) => {
                            let i2 = i - DIJ[dir].0;
                            let j2 = j - DIJ[dir].1;
                            if i2 < n && j2 < m {
                                if crt[i2][j2] != P::Empty {
                                    read[i2][j2] = true;
                                    let i3 = i + DIJ[dir].0;
                                    let j3 = j + DIJ[dir].1;
                                    if i3 >= n || j3 >= m {
                                        return Sim {
                                            score: 0,
                                            ret: P::Empty,
                                            err: format!("Out of bounds: ({}, {})", i, j),
                                            log,
                                        };
                                    }
                                    if write[i3][j3] != P::Empty {
                                        return Sim {
                                            score: 0,
                                            ret: P::Empty,
                                            err: format!("Conflict write: ({}, {})", i3, j3),
                                            log,
                                        };
                                    }
                                    write[i3][j3] = crt[i2][j2].clone();
                                }
                            }
                        }
                        P::Op(op) => {
                            if i > 0
                                && j > 0
                                && crt[i - 1][j] != P::Empty
                                && crt[i][j - 1] != P::Empty
                            {
                                if let (P::Num(x), P::Num(y)) =
                                    (crt[i][j - 1].clone(), crt[i - 1][j].clone())
                                {
                                    let ret = match op {
                                        '+' => x + y,
                                        '-' => x - y,
                                        '*' => x * y,
                                        '/' => {
                                            if y == BigInt::from(0) {
                                                return Sim {
                                                    score: 0,
                                                    ret: P::Empty,
                                                    err: format!(
                                                        "Division by zero: ({}, {})",
                                                        i, j
                                                    ),
                                                    log,
                                                };
                                            } else {
                                                x / y
                                            }
                                        }
                                        '%' => {
                                            if y == BigInt::from(0) {
                                                return Sim {
                                                    score: 0,
                                                    ret: P::Empty,
                                                    err: format!(
                                                        "Division by zero: ({}, {})",
                                                        i, j
                                                    ),
                                                    log,
                                                };
                                            } else {
                                                x % y.abs()
                                            }
                                        }
                                        _ => {
                                            unreachable!()
                                        }
                                    };
                                    if i + 1 >= n || j + 1 >= m {
                                        return Sim {
                                            score: 0,
                                            ret: P::Empty,
                                            err: format!("Out of bounds: ({}, {})", i, j),
                                            log,
                                        };
                                    }
                                    if write[i + 1][j] != P::Empty {
                                        return Sim {
                                            score: 0,
                                            ret: P::Empty,
                                            err: format!("Conflict write: ({}, {})", i + 1, j),
                                            log,
                                        };
                                    }
                                    if write[i][j + 1] != P::Empty {
                                        return Sim {
                                            score: 0,
                                            ret: P::Empty,
                                            err: format!("Conflict write: ({}, {})", i, j + 1),
                                            log,
                                        };
                                    }
                                    read[i - 1][j] = true;
                                    read[i][j - 1] = true;
                                    write[i + 1][j] = P::Num(ret.clone());
                                    write[i][j + 1] = P::Num(ret);
                                }
                            }
                        }
                        P::Eq | P::Neq => {
                            if i > 0
                                && j > 0
                                && crt[i - 1][j] != P::Empty
                                && crt[i][j - 1] != P::Empty
                                && (crt[i][j] == P::Eq && crt[i - 1][j] == crt[i][j - 1]
                                    || crt[i][j] == P::Neq && crt[i - 1][j] != crt[i][j - 1])
                            {
                                if i + 1 >= n || j + 1 >= m {
                                    return Sim {
                                        score: 0,
                                        ret: P::Empty,
                                        err: format!("Out of bounds: ({}, {})", i, j),
                                        log,
                                    };
                                }
                                if write[i + 1][j] != P::Empty {
                                    return Sim {
                                        score: 0,
                                        ret: P::Empty,
                                        err: format!("Conflict write: ({}, {})", i + 1, j),
                                        log,
                                    };
                                }
                                if write[i][j + 1] != P::Empty {
                                    return Sim {
                                        score: 0,
                                        ret: P::Empty,
                                        err: format!("Conflict write: ({}, {})", i, j + 1),
                                        log,
                                    };
                                }
                                read[i - 1][j] = true;
                                read[i][j - 1] = true;
                                write[i + 1][j] = crt[i][j - 1].clone();
                                write[i][j + 1] = crt[i - 1][j].clone();
                            }
                        }
                        _ => (),
                    }
                }
            }
            for i in 0..n {
                for j in 0..m {
                    if read[i][j] {
                        next[i][j] = P::Empty;
                    }
                }
            }
            for i in 0..n {
                for j in 0..m {
                    if write[i][j] != P::Empty {
                        if next[i][j] == P::Submit {
                            return Sim {
                                score: (n * m * (times.len() + 1)) as i64,
                                ret: write[i][j].clone(),
                                err: String::new(),
                                log,
                            };
                        }
                        next[i][j] = write[i][j].clone();
                    }
                }
            }
            next_t = t + 1;
        }
        crt = next;
        t = next_t;
    }
    return Sim {
        score: 0,
        ret: P::Empty,
        err: "Time limit exceeded".to_string(),
        log,
    };
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val.setmin(1.0);
    val.setmax(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        r.round() as i32,
        g.round() as i32,
        b.round() as i32
    )
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new(title))
}

pub fn vis(sim: &Sim, t: usize) -> String {
    let n = sim.log[t].1.len();
    let m = sim.log[t].1[0].len();
    let D = 600 / n.max(m);
    let W = m * D;
    let H = n * D;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    for i in 0..n {
        for j in 0..m {
            let s = sim.log[t].1[i][j].to_string();
            doc = doc.add(
                group(s.clone()).add(rect(j * D, i * D, D, D, "white")).add(
                    svg::node::element::Text::new(s.clone())
                        .set("x", j * D + D / 2)
                        .set("y", i * D + D / 2)
                        .set("font-size", D as f64 / s.len() as f64)
                        .set("fill", "black"),
                ),
            );
        }
    }
    for i in 0..=n {
        doc = doc.add(
            Line::new()
                .set("x1", 0)
                .set("y1", i * D)
                .set("x2", W)
                .set("y2", i * D)
                .set("stroke", "black")
                .set("stroke-width", 1),
        );
    }
    for j in 0..=m {
        doc = doc.add(
            Line::new()
                .set("x1", j * D)
                .set("y1", 0)
                .set("x2", j * D)
                .set("y2", H)
                .set("stroke", "black")
                .set("stroke-width", 1),
        );
    }

    doc.to_string()
}
