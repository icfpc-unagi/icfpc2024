#![allow(non_snake_case)]

use itertools::Itertools;
use rand::prelude::*;
use rustc_hash::FxHashSet;
use solution::*;

const TL: f64 = 600.0;

fn get_order(input: &Input) -> Vec<usize> {
    if let Ok(best) = std::env::var("BEST") {
        return solution::read_order(input, &best);
    }
    if input.ps.len() > 10000 {
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
    let mut g = mat![1000000000; input.ps.len() + 3; input.ps.len() + 3];
    for i in 0..input.ps.len() {
        for j in 0..input.ps.len() {
            g[i][j] = (input.ps[i].0 - input.ps[j].0).abs() + (input.ps[i].1 - input.ps[j].1).abs();
        }
        g[i][input.ps.len()] = input.ps[i].0.abs() + input.ps[i].1.abs();
        g[input.ps.len()][i] = g[i][input.ps.len()];
        g[i][input.ps.len() + 1] = 0;
        g[input.ps.len() + 1][i] = 0;
    }
    g[input.ps.len()][input.ps.len() + 2] = 0;
    g[input.ps.len() + 2][input.ps.len()] = 0;
    g[input.ps.len() + 1][input.ps.len() + 2] = 0;
    g[input.ps.len() + 2][input.ps.len() + 1] = 0;
    let order = [input.ps.len()]
        .iter()
        .copied()
        .chain(0..input.ps.len())
        .chain([input.ps.len() + 1, input.ps.len() + 2, input.ps.len()])
        .collect_vec();
    let mut order = tsp::solve(
        &g,
        &order,
        60.0,
        &mut rand_pcg::Pcg64Mcg::seed_from_u64(4932080),
    );
    if order[1] >= input.ps.len() {
        order.reverse();
    }
    order[1..1 + input.ps.len()].to_vec()
}

fn main() {
    get_time();
    let input = read_input();
    let order = get_order(&input);
    let mut beam = vec![State {
        visited: vec![false; input.ps.len()],
        p: (0, 0),
        v: (0, 0),
        t: 0,
        id: !0,
    }];
    // for i in 1..input.ps.len() {
    //     eprintln!(
    //         "{}\t{}",
    //         input.ps[order[i]].0 - input.ps[order[i - 1]].0,
    //         input.ps[order[i]].1 - input.ps[order[i - 1]].1
    //     );
    // }
    let mut trace = Trace::new();
    let stime = get_time();
    let mut rng = rand_pcg::Pcg64::seed_from_u64(thread_rng().gen());
    for k in 0..input.ps.len() {
        let mut next = vec![];
        let mut w = 0;
        for state in beam {
            w += 1;
            let i = order[k];
            let mut T = 0;
            loop {
                let dx = input.ps[i].0 - state.p.0;
                let dy = input.ps[i].1 - state.p.1;
                if state.v.0 * T - T * (T + 1) / 2 <= dx && dx <= state.v.0 * T + T * (T + 1) / 2 {
                    if state.v.1 * T - T * (T + 1) / 2 <= dy
                        && dy <= state.v.1 * T + T * (T + 1) / 2
                    {
                        break;
                    }
                }
                T += 1;
            }
            for _ in 0..2 {
                let dx = input.ps[i].0 - state.p.0;
                let dy = input.ps[i].1 - state.p.1;
                if state.v.0 * T - T * (T + 1) / 2 <= dx && dx <= state.v.0 * T + T * (T + 1) / 2 {
                    if state.v.1 * T - T * (T + 1) / 2 <= dy
                        && dy <= state.v.1 * T + T * (T + 1) / 2
                    {
                        let mut used = FxHashSet::default();
                        for _ in 0..1000 {
                            let mut p = state.p;
                            let mut v = state.v;
                            let mut id = state.id;
                            let mut tmp = vec![];
                            for t in 0..T {
                                let cand = (-1..=1)
                                    .filter(|&a| {
                                        p.0 + (v.0 + a) * (T - t) - (T - t) * (T - t - 1) / 2
                                            <= input.ps[i].0
                                            && input.ps[i].0
                                                <= p.0
                                                    + (v.0 + a) * (T - t)
                                                    + (T - t) * (T - t - 1) / 2
                                    })
                                    .collect_vec();
                                let dvx = *cand.choose(&mut rng).unwrap();
                                let cand = (-1..=1)
                                    .filter(|&a| {
                                        p.1 + (v.1 + a) * (T - t) - (T - t) * (T - t - 1) / 2
                                            <= input.ps[i].1
                                            && input.ps[i].1
                                                <= p.1
                                                    + (v.1 + a) * (T - t)
                                                    + (T - t) * (T - t - 1) / 2
                                    })
                                    .collect_vec();
                                let dvy = *cand.choose(&mut rng).unwrap();
                                v.0 += dvx;
                                v.1 += dvy;
                                p.0 += v.0;
                                p.1 += v.1;
                                tmp.push((dvy + 1) * 3 + dvx + 1 + 1);
                            }
                            assert_eq!(p, input.ps[i]);
                            if used.insert(v) {
                                for mv in tmp {
                                    id = trace.add(mv, id);
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
                        }
                    }
                }
                T += 1;
            }
            if (get_time() - stime) > TL * (k + 1) as f64 / input.ps.len() as f64 {
                break;
            }
        }
        next.sort_by_key(|s| s.t + s.v.0.abs() + s.v.1.abs());
        eprintln!("{} / {}: w = {}, t = {}", k, input.ps.len(), w, next[0].t);
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
    let b = beam.iter().min_by_key(|s| s.t).unwrap();
    for mv in trace.get(b.id) {
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
