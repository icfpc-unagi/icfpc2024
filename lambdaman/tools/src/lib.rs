#![allow(non_snake_case, unused_macros)]

use std::ops::RangeBounds;
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

const INPUT: [&str; 21] = [
    include_str!("../input/lambdaman1.txt"),
    include_str!("../input/lambdaman2.txt"),
    include_str!("../input/lambdaman3.txt"),
    include_str!("../input/lambdaman4.txt"),
    include_str!("../input/lambdaman5.txt"),
    include_str!("../input/lambdaman6.txt"),
    include_str!("../input/lambdaman7.txt"),
    include_str!("../input/lambdaman8.txt"),
    include_str!("../input/lambdaman9.txt"),
    include_str!("../input/lambdaman10.txt"),
    include_str!("../input/lambdaman11.txt"),
    include_str!("../input/lambdaman12.txt"),
    include_str!("../input/lambdaman13.txt"),
    include_str!("../input/lambdaman14.txt"),
    include_str!("../input/lambdaman15.txt"),
    include_str!("../input/lambdaman16.txt"),
    include_str!("../input/lambdaman17.txt"),
    include_str!("../input/lambdaman18.txt"),
    include_str!("../input/lambdaman19.txt"),
    include_str!("../input/lambdaman20.txt"),
    include_str!("../input/lambdaman21.txt"),
];

#[derive(Clone, Debug)]
pub struct Input {
    pub board: Vec<Vec<char>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let mut board = vec![];
    for line in f.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        board.push(line.chars().collect());
    }
    Input { board }
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

pub struct Output {
    pub score: i64,
    pub out: Vec<u8>,
}

pub fn parse_output(_input: &Input, f: &str) -> Result<Output, String> {
    use icfpc2024::eval::*;
    let out = f.trim().to_owned();
    let score = out.len() as i64;
    let Value::Str(mut out) = eval(&out) else {
        return Err("Not evaluated as string".to_owned());
    };
    for _ in 0..2 {
        let Some(p) = out.iter().position(|&c| c == b' ') else {
            return Err("Illegal format".to_owned());
        };
        out = out[p + 1..].to_vec();
    }
    Ok(Output { score, out })
}

pub fn gen(seed: u64) -> Input {
    let input = parse_input(&INPUT[seed as usize - 1]);
    input
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, (&out.out, out.score));
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(
    input: &Input,
    (out, score): (&[u8], i64),
) -> (i64, String, ((usize, usize), Vec<Vec<bool>>)) {
    let mut p = (0, 0);
    for i in 0..input.board.len() {
        for j in 0..input.board[i].len() {
            if input.board[i][j] == 'L' {
                p = (i, j);
            }
        }
    }
    let mut visited = mat![false; input.board.len(); input.board[0].len()];
    for mv in out {
        let bk = p;
        match mv {
            b'U' => {
                p.0 -= 1;
            }
            b'D' => {
                p.0 += 1;
            }
            b'L' => {
                p.1 -= 1;
            }
            b'R' => {
                p.1 += 1;
            }
            _ => {
                return (0, format!("Invalid move: {}", *mv as char), (p, visited));
            }
        }
        if p.0 >= input.board.len() || p.1 >= input.board[0].len() {
            return (0, "Out of bounds".to_owned(), (p, visited));
        }
        if input.board[p.0][p.1] == '#' {
            p = bk;
        }
        visited[p.0][p.1] = true;
    }
    let mut ok = true;
    for i in 0..input.board.len() {
        for j in 0..input.board[i].len() {
            if input.board[i][j] == '.' && !visited[i][j] {
                ok = false;
            }
        }
    }
    if !ok {
        return (0, "Not all cells visited".to_owned(), (p, visited));
    }
    (score, String::new(), (p, visited))
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

pub fn vis_default(input: &Input, out: &Output) -> (i64, String, String) {
    let (mut score, err, svg) = vis(input, (&out.out, out.score));
    if err.len() > 0 {
        score = 0;
    }
    (score, err, svg)
}

pub fn vis(input: &Input, out: (&[u8], i64)) -> (i64, String, String) {
    let D = 600 / input.board.len().max(input.board[0].len());
    let W = D * input.board[0].len();
    let H = D * input.board.len();
    let (score, err, (p, visited)) = compute_score_details(input, out);
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    for i in 0..input.board.len() {
        for j in 0..input.board[i].len() {
            let fill = match input.board[i][j] {
                '#' => "chocolate",
                '.' => {
                    if visited[i][j] {
                        "white"
                    } else {
                        "gray"
                    }
                }
                'L' => "white",
                _ => unreachable!(),
            };
            doc = doc.add(group(format!("({}, {})", i, j)).add(rect(j * D, i * D, D, D, fill)));
        }
    }
    doc = doc.add(group(format!("({}, {})", p.0, p.1)).add(rect(p.1 * D, p.0 * D, D, D, "red")));
    if D > 10 {
        for i in 0..=input.board.len() {
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
        for j in 0..=input.board[0].len() {
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
    }
    (score, err, doc.to_string())
}
