#![allow(non_snake_case)]

use itertools::Itertools;
use rand::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use solution::*;

const TL: f64 = 3600.0;
const DT: i64 = 3;

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

// dt秒後にちょうどdだけ進むための加速方法と終速度を列挙
fn listup(dt: i64, d: i64) -> Vec<(i64, Vec<i64>)> {
    let mut trace = Trace::new();
    let mut crt = vec![(d, 0, !0)];
    for t in 0..dt {
        let mut next = vec![];
        let mut visited = FxHashSet::default();
        for &(d, v, id) in &crt {
            for a in -1..=1 {
                if (v + a) * (dt - t) - (dt - t) * (dt - t - 1) / 2 <= d
                    && d <= (v + a) * (dt - t) + (dt - t) * (dt - t - 1) / 2
                {
                    let v = v + a;
                    let d = d - v;
                    if visited.insert((d, v)) {
                        next.push((d, v, trace.add(a, id)));
                    }
                }
            }
        }
        crt = next;
    }
    crt.into_iter()
        .map(|(_, v, id)| (v, trace.get(id)))
        .collect()
}

fn main() {
    get_time();
    let mut input = read_input();
    input.ps.sort();
    input.ps.dedup();
    let order = get_order(&input);
    let mut trace = Trace::new();
    let mut beam = vec![State {
        t: 0,
        v: (0, 0),
        id: !0,
    }];
    let best = if let Ok(best) = std::env::var("BEST") {
        read_output(&best)
    } else {
        vec![]
    };
    let mut best_state = State {
        v: (0, 0),
        t: 0,
        id: !0,
    };
    let stime = get_time();
    let mut cache_vs = FxHashMap::default();
    for k in 0..input.ps.len() {
        let p = if k == 0 {
            (0, 0)
        } else {
            input.ps[order[k - 1]]
        };
        let q = input.ps[order[k]];
        let mut list = vec![];
        let dx = q.0 - p.0;
        let dy = q.1 - p.1;
        for s in 0..beam.len() {
            let mut T = 0;
            let v = beam[s].v;
            loop {
                if v.0 * T - T * (T + 1) / 2 <= dx && dx <= v.0 * T + T * (T + 1) / 2 {
                    if v.1 * T - T * (T + 1) / 2 <= dy && dy <= v.1 * T + T * (T + 1) / 2 {
                        break;
                    }
                }
                T += 1;
            }
            for T in T..T + DT {
                if v.0 * T - T * (T + 1) / 2 <= dx && dx <= v.0 * T + T * (T + 1) / 2 {
                    if v.1 * T - T * (T + 1) / 2 <= dy && dy <= v.1 * T + T * (T + 1) / 2 {
                        list.push((beam[s].t + T, s));
                    }
                }
            }
        }
        list.sort();
        let mut next = vec![];
        let mut visited = FxHashSet::default();
        'list: for (T, s) in list {
            if !cache_vs.contains_key(&(T - beam[s].t, dx - beam[s].v.0 * (T - beam[s].t))) {
                cache_vs.insert(
                    (T - beam[s].t, dx - beam[s].v.0 * (T - beam[s].t)),
                    listup(T - beam[s].t, dx - beam[s].v.0 * (T - beam[s].t)),
                );
            }
            if !cache_vs.contains_key(&(T - beam[s].t, dy - beam[s].v.1 * (T - beam[s].t))) {
                cache_vs.insert(
                    (T - beam[s].t, dy - beam[s].v.1 * (T - beam[s].t)),
                    listup(T - beam[s].t, dy - beam[s].v.1 * (T - beam[s].t)),
                );
            }
            let vx = &cache_vs[&(T - beam[s].t, dx - beam[s].v.0 * (T - beam[s].t))];
            let vy = &cache_vs[&(T - beam[s].t, dy - beam[s].v.1 * (T - beam[s].t))];
            for (vx, ax) in vx.iter() {
                for (vy, ay) in vy.iter() {
                    let vx = beam[s].v.0 + *vx;
                    let vy = beam[s].v.1 + *vy;
                    if visited.insert((vx, vy)) {
                        let mut id = beam[s].id;
                        for i in 0..ax.len() {
                            id = trace.add((ay[i] + 1) * 3 + (ax[i] + 1) + 1, id);
                        }
                        next.push(State {
                            t: T,
                            v: (vx, vy),
                            id,
                        });
                        if beam.len() > 1000000 {
                            break;
                        }
                    }
                }
            }
            if get_time() - stime > TL * (k + 1) as f64 / input.ps.len() as f64 {
                break 'list;
            }
        }
        if !best.is_empty() {
            let mut p = p;
            while p != q {
                let mv = best[best_state.t as usize];
                best_state.v.0 += (mv - 1) % 3 - 1;
                best_state.v.1 += (mv - 1) / 3 - 1;
                p.0 += best_state.v.0;
                p.1 += best_state.v.1;
                best_state.id = trace.add(mv, best_state.id);
                best_state.t += 1;
            }
            if let Some(i) = (0..next.len()).find(|&i| next[i].v == best_state.v) {
                if next[i].t > best_state.t {
                    next.remove(i);
                    let j = next
                        .iter()
                        .position(|s| s.t > best_state.t)
                        .unwrap_or(next.len());
                    next.insert(j, best_state.clone());
                }
            } else {
                let j = next
                    .iter()
                    .position(|s| s.t > best_state.t)
                    .unwrap_or(next.len());
                next.insert(j, best_state.clone());
            }
        }
        beam = next;
        eprintln!(
            "{} / {}: w = {}, t = {} ({})",
            k,
            input.ps.len(),
            beam.len(),
            beam[0].t,
            best_state.t
        );
        let live = beam.iter().map(|s| s.id).collect_vec();
        let ids = trace.compact(&live);
        for s in beam.iter_mut() {
            s.id = if s.id == !0 { !0 } else { ids[s.id] };
        }
    }
    let out = trace.get(beam[0].id);
    for mv in out {
        println!("{}", mv);
    }
    eprintln!("Time = {:.3}", get_time());
}

#[derive(Clone, Debug)]
struct State {
    t: i64,
    v: (i64, i64),
    id: usize,
}
