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
            eprintln!("dist: {}", dist);
        }
        if visited.len() > 100000000 {
            panic!();
        }
        if (p, v) == (tp, tv) {
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

fn main() {
    get_time();
    let input = read_input();
    let order = solution::read_order(&input, "all/best");
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
    for (x, y) in move_x.into_iter().zip(move_y.into_iter()) {
        out.push((y + 1) * 3 + (x + 1) + 1);
    }
    let mut v = target_v;
    for i in 1..order.len() {
        let v2 = (
            input.ps[order[i]].0 - input.ps[order[i - 1]].0,
            input.ps[order[i]].1 - input.ps[order[i - 1]].1,
        );
        let a = (v2.0 - v.0, v2.1 - v.1);
        if a.0.abs() > 1 || a.1.abs() > 1 {
            panic!();
        }
        out.push((a.1 + 1) * 3 + (a.0 + 1) + 1);
        v = v2;
    }
    for mv in out {
        println!("{}", mv);
    }
}
