#![allow(non_snake_case)]

use rustc_hash::FxHashMap;
use solution::*;
use std::collections::VecDeque;

fn solve(tp: i64, tv: i64) -> Vec<i64> {
    dbg!(tp, tv);
    let mut visited = FxHashMap::default();
    visited.insert((0, 0), !0);
    let mut que = VecDeque::new();
    que.push_back((0, 0, 0, !0));
    let mut trace = Trace::new();
    let mut max_dist = 0;
    while let Some((p, v, dist, id)) = que.pop_front() {
        if max_dist.setmax(dist) {
            eprintln!("dist: {} (size = {})", dist, visited.len());
        }
        if visited.len() > 100000000 {
            panic!();
        }
        if p == tp && (v - tv).abs() <= 1 {
            let out = trace.get(id);
            eprintln!("found: {}", out.len());
            return trace.get(id);
        }
        for a in -1..=1 {
            let v = v + a;
            let p = p + v;
            if !visited.contains_key(&(p, v)) {
                let id = trace.add(a, id);
                visited.insert((p, v), id);
                que.push_back((p, v, dist + 1, id));
            }
        }
    }
    unreachable!()
}

pub fn compute_dist(input: &Input, order: &[usize]) -> i64 {
    let mut dist = input.ps[order[0]].0.abs() + input.ps[order[0]].1.abs();
    for i in 1..order.len() {
        dist += (input.ps[order[i]].0 - input.ps[order[i - 1]].0).abs()
            + (input.ps[order[i]].1 - input.ps[order[i - 1]].1).abs();
    }
    dist
}

pub fn get_order(input: &Input) -> Vec<usize> {
    let mut order = vec![0];
    for i in 1..input.ps.len() {
        let mut min = i64::MAX;
        let mut p = 0;
        for j in 0..order.len() {
            let prev = if j == 0 {
                (0, 0)
            } else {
                input.ps[order[j - 1]]
            };
            let next = input.ps[order[j]];
            let d = (input.ps[i].0 - prev.0).abs()
                + (input.ps[i].1 - prev.1).abs()
                + (input.ps[i].0 - next.0).abs()
                + (input.ps[i].1 - next.1).abs()
                - (next.0 - prev.0).abs()
                - (next.1 - prev.1).abs();
            if min.setmin(d) {
                p = j;
            }
        }
        let d = (input.ps[i].0 - input.ps[order[order.len() - 1]].0).abs()
            + (input.ps[i].1 - input.ps[order[order.len() - 1]].1).abs();
        if min.setmin(d) {
            p = order.len();
        }
        order.insert(p, i);
    }
    return order;
}

fn main() {
    get_time();
    let input = read_input();
    let order = solution::read_order(&input, "all/best");
    // for i in 0..order.len() - 1 {
    //     println!(
    //         "{}, {}",
    //         input.ps[order[i + 1]].0 - input.ps[order[i]].0,
    //         input.ps[order[i + 1]].1 - input.ps[order[i]].1
    //     );
    // }
    // return;
    let target_p = input.ps[order[0]];
    let target_v = (
        input.ps[order[1]].0 - input.ps[order[0]].0,
        input.ps[order[1]].1 - input.ps[order[0]].1,
    );
    let mut move_x = solve(target_p.0, target_v.0);
    let mut move_y = solve(target_p.1, target_v.1);
    while move_x.len() < move_y.len() {
        move_x.insert(0, 0);
    }
    while move_x.len() > move_y.len() {
        move_y.insert(0, 0);
    }
    let mut out = vec![];
    let mut v = (0, 0);
    for (x, y) in move_x.into_iter().zip(move_y.into_iter()) {
        out.push((y + 1) * 3 + (x + 1) + 1);
        v.0 += x;
        v.1 += y;
    }
    for i in 1..order.len() {
        let v2 = (
            input.ps[order[i]].0 - input.ps[order[i - 1]].0,
            input.ps[order[i]].1 - input.ps[order[i - 1]].1,
        );
        let a = (v2.0 - v.0, v2.1 - v.1);
        if a.0.abs() > 1 || a.1.abs() > 1 {
            panic!("{:?}", a);
        }
        out.push((a.1 + 1) * 3 + (a.0 + 1) + 1);
        v = v2;
    }
    for mv in out {
        println!("{}", mv);
    }
}
