#![allow(non_snake_case)]

use itertools::Itertools;
use rustc_hash::FxHashSet;
use solution::*;

const TL: f64 = 600.0;

fn main() {
    get_time();
    let input = read_input();
    let mut beam = vec![State {
        visited: vec![false; input.ps.len()],
        p: (0, 0),
        v: (0, 0),
        t: 0,
        id: !0,
    }];
    let mut trace = Trace::new();
    for k in 0..input.ps.len() {
        let mut next = vec![];
        for state in beam {
            let target = (0..input.ps.len())
                .filter(|&i| !state.visited[i])
                .sorted_by_key(|&i| {
                    (input.ps[i].0 - state.p.0 - state.v.0).abs()
                        + (input.ps[i].1 - state.p.1 - state.v.1).abs()
                })
                .take(2)
                .collect_vec();
            for i in target {
                let mut T = 0;
                loop {
                    let dx = input.ps[i].0 - state.p.0;
                    let dy = input.ps[i].1 - state.p.1;
                    if state.v.0 * T - T * (T + 1) / 2 <= dx
                        && dx <= state.v.0 * T + T * (T + 1) / 2
                    {
                        if state.v.1 * T - T * (T + 1) / 2 <= dy
                            && dy <= state.v.1 * T + T * (T + 1) / 2
                        {
                            break;
                        }
                    }
                    T += 1;
                }
                let mut p = state.p;
                let mut v = state.v;
                let mut id = state.id;
                for t in 0..T {
                    let dvx = if p.0 + v.0 * (T - t) < input.ps[i].0 {
                        1
                    } else if p.0 + v.0 * (T - t) > input.ps[i].0 {
                        -1
                    } else {
                        0
                    };
                    let dvy = if p.1 + v.1 * (T - t) < input.ps[i].1 {
                        1
                    } else if p.1 + v.1 * (T - t) > input.ps[i].1 {
                        -1
                    } else {
                        0
                    };
                    v.0 += dvx;
                    v.1 += dvy;
                    p.0 += v.0;
                    p.1 += v.1;
                    id = trace.add((dvy + 1) * 3 + dvx + 1 + 1, id);
                }
                let mut visited = state.visited.clone();
                visited[i] = true;
                next.push(State {
                    visited,
                    p,
                    v,
                    t: state.t + T,
                    id,
                });
            }
            if get_time() > TL * (k + 1) as f64 / input.ps.len() as f64 {
                break;
            }
        }
        next.sort_by_key(|s| s.t);
        beam = vec![];
        let mut used = FxHashSet::default();
        for s in next {
            let h = (s.visited.clone(), s.p, s.v);
            if used.contains(&h) {
                continue;
            }
            used.insert(h);
            beam.push(s);
            if beam.len() >= 100000 {
                break;
            }
        }
        let live = beam.iter().map(|s| s.id).collect_vec();
        let ids = trace.compact(&live);
        for s in beam.iter_mut() {
            s.id = if s.id == !0 { !0 } else { ids[s.id] };
        }
    }
    for mv in trace.get(beam[0].id) {
        println!("{}", mv);
    }
    eprintln!("Time = {:.3}", get_time());
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    visited: Vec<bool>,
    p: (i64, i64),
    v: (i64, i64),
    t: i64,
    id: usize,
}
