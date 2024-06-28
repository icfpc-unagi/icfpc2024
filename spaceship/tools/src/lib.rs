#![allow(non_snake_case, unused_macros)]

use std::{collections::BTreeMap, ops::RangeBounds};
use svg::node::element::{Circle, Group, Line, Rectangle, Style, Title};

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

const INPUT: [&str; 24] = [
    include_str!("../input/spaceship1.txt"),
    include_str!("../input/spaceship2.txt"),
    include_str!("../input/spaceship3.txt"),
    include_str!("../input/spaceship4.txt"),
    include_str!("../input/spaceship5.txt"),
    include_str!("../input/spaceship6.txt"),
    include_str!("../input/spaceship7.txt"),
    include_str!("../input/spaceship8.txt"),
    include_str!("../input/spaceship9.txt"),
    include_str!("../input/spaceship10.txt"),
    include_str!("../input/spaceship11.txt"),
    include_str!("../input/spaceship12.txt"),
    include_str!("../input/spaceship13.txt"),
    include_str!("../input/spaceship14.txt"),
    include_str!("../input/spaceship15.txt"),
    include_str!("../input/spaceship16.txt"),
    include_str!("../input/spaceship17.txt"),
    include_str!("../input/spaceship18.txt"),
    include_str!("../input/spaceship19.txt"),
    include_str!("../input/spaceship20.txt"),
    include_str!("../input/spaceship21.txt"),
    include_str!("../input/spaceship22.txt"),
    include_str!("../input/spaceship23.txt"),
    include_str!("../input/spaceship24.txt"),
];

#[derive(Clone, Debug)]
pub struct Input {
    pub ps: Vec<(i32, i32)>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, y) in &self.ps {
            writeln!(f, "{} {}", x, y)?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let mut ps = vec![];
    for line in f.lines() {
        let line = line.trim();
        if line.len() > 0 {
            let ss = line.split_whitespace().collect::<Vec<_>>();
            ps.push((ss[0].parse().unwrap(), ss[1].parse().unwrap()));
        }
    }
    Input { ps }
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
    pub out: Vec<usize>,
}

pub fn parse_output(_input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    for line in f.lines() {
        let line = line.trim();
        if line.len() > 0 {
            out.push(read(Some(line), 1..=9)?);
        }
    }
    Ok(Output { out })
}

pub fn gen(seed: u64) -> Input {
    let input = parse_input(&INPUT[seed as usize]);
    input
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(
    input: &Input,
    out: &[usize],
) -> (i64, String, ((i32, i32), Vec<bool>, Vec<(i32, i32)>)) {
    let mut p = (0, 0);
    let mut v = (0, 0);
    let mut id = BTreeMap::new();
    for i in 0..input.ps.len() {
        id.entry(input.ps[i]).or_insert(vec![]).push(i);
    }
    let mut visited = vec![false; input.ps.len()];
    let mut route = vec![];
    if let Some(is) = id.get(&p) {
        for i in is {
            visited[*i] = true;
        }
    }
    for &mv in out {
        route.push(p);
        let dx = (mv as i32 - 1) % 3 - 1;
        let dy = (mv as i32 - 1) / 3 - 1;
        v.0 += dx;
        v.1 += dy;
        p.0 += v.0;
        p.1 += v.1;
        if let Some(is) = id.get(&p) {
            for i in is {
                visited[*i] = true;
            }
        }
    }
    route.push(p);
    for i in 0..input.ps.len() {
        if !visited[i] {
            return (
                0,
                format!("Point {} is not visited", i),
                (p, visited, route),
            );
        }
    }
    (out.len() as i64, String::new(), (p, visited, route))
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
    let (mut score, err, svg) = vis(input, &out.out);
    if err.len() > 0 {
        score = 0;
    }
    (score, err, svg)
}

pub fn vis(input: &Input, out: &[usize]) -> (i64, String, String) {
    let W = 600;
    let H = 600;
    let (score, err, (p, visited, mut route)) = compute_score_details(input, &out);
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for &(x, y) in &input.ps {
        min.setmin(x);
        min.setmin(y);
        max.setmax(x);
        max.setmax(y);
    }
    let w = max - min;
    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set(
            "viewBox",
            (
                min - (w + 19) / 20,
                min - (w + 19) / 20,
                w + (w + 19) / 20 * 2,
                w + (w + 19) / 20 * 2,
            ),
        )
        .set("width", W)
        .set("height", H)
        .set("style", "background-color:white");
    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central;}}"
    )));
    if route.len() >= 20000 {
        route = route.iter().step_by(route.len() / 10000).copied().collect();
    }
    for i in 1..route.len() {
        doc = doc.add(
            Line::new()
                .set("x1", route[i - 1].0)
                .set("y1", route[i - 1].1)
                .set("x2", route[i].0)
                .set("y2", route[i].1)
                .set("stroke", "blue")
                .set("stroke-width", w as f64 / 600.0),
        );
    }
    for i in 0..input.ps.len() {
        let (x, y) = input.ps[i];
        let fill = if visited[i] { "gray" } else { "black" };
        doc = doc.add(
            group(format!("({}, {})", x, y)).add(
                Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", w as f64 / 200.0)
                    .set("fill", fill),
            ),
        );
    }
    doc = doc.add(
        Circle::new()
            .set("cx", p.0)
            .set("cy", p.1)
            .set("r", w as f64 / 100.0)
            .set("fill", "red"),
    );
    (score, err, doc.to_string())
}
