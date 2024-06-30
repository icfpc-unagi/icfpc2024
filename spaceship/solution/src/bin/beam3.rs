#![allow(non_snake_case)]

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use solution::*;

const TL: f64 = 6.0 * 3600.0;
const V_PENALITY: i64 = 1;

// 訪問済みの頂点数で区切ってビームサーチをする
// 訪問順の多様性を重視

fn main() {
    get_time();
    let input = read_input();
    let best = if let Ok(best) = std::env::var("BEST") {
        read_output(&best)
    } else {
        vec![]
    };
    let mut best_state = State {
        visited: FixedBitSet::with_capacity(input.ps.len()),
        p: (0, 0),
        v: (0, 0),
        t: 0,
        id: !0,
    };
    let mut beam = vec![State {
        visited: FixedBitSet::with_capacity(input.ps.len()),
        p: (0, 0),
        v: (0, 0),
        t: 0,
        id: !0,
    }];
    let mut trace = Trace::new();
    for k in 0..input.ps.len() {
        let mut next = vec![];
        let mut w = 0;
        for state in beam {
            w += 1;
            let target = {
                let mut list = BoundedSortedList::new(10);
                for i in 0..input.ps.len() {
                    if !state.visited[i] {
                        list.insert(
                            (input.ps[i].0 - state.p.0).abs() + (input.ps[i].1 - state.p.1).abs(),
                            i,
                        );
                    }
                }
                list.list().into_iter().map(|(_, i)| i).collect_vec()
            };
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
                let mut used = FxHashSet::default();
                for _ in 0..1 {
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
                                        <= p.0 + (v.0 + a) * (T - t) + (T - t) * (T - t - 1) / 2
                            })
                            .collect_vec();
                        let dvx = if v.0 > 0 {
                            cand[cand.len() - 1]
                        } else if v.0 < 0 {
                            cand[0]
                        } else {
                            cand[cand.len() / 2]
                        };
                        let cand = (-1..=1)
                            .filter(|&a| {
                                p.1 + (v.1 + a) * (T - t) - (T - t) * (T - t - 1) / 2
                                    <= input.ps[i].1
                                    && input.ps[i].1
                                        <= p.1 + (v.1 + a) * (T - t) + (T - t) * (T - t - 1) / 2
                            })
                            .collect_vec();
                        let dvy = if v.1 > 0 {
                            cand[cand.len() - 1]
                        } else if v.1 < 0 {
                            cand[0]
                        } else {
                            cand[cand.len() / 2]
                        };
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
                        visited.insert(i);
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
            if get_time() > TL * (k + 1) as f64 / input.ps.len() as f64 {
                break;
            }
        }
        if !best.is_empty() {
            loop {
                if let Some(i) = (0..input.ps.len())
                    .find(|&i| !best_state.visited[i] && input.ps[i] == best_state.p)
                {
                    best_state.visited.insert(i);
                    break;
                }
                let mv = best[best_state.t as usize];
                best_state.v.0 += (mv - 1) % 3 - 1;
                best_state.v.1 += (mv - 1) / 3 - 1;
                best_state.p.0 += best_state.v.0;
                best_state.p.1 += best_state.v.1;
                best_state.id = trace.add(mv, best_state.id);
                best_state.t += 1;
            }
        }
        next.push(best_state.clone());
        if k + 1 == input.ps.len() {
            next.sort_by_key(|s| s.t);
        } else {
            next.sort_by_key(|s| s.t + (s.v.0.abs() + s.v.1.abs()) * V_PENALITY);
        }
        eprintln!(
            "{} / {}: w = {}, t = {} ({})",
            k,
            input.ps.len(),
            w,
            next[0].t,
            best_state.t
        );
        beam = vec![];
        let mut used = FxHashSet::default();
        for s in next {
            let h = (s.p, s.visited.clone());
            if used.contains(&h) {
                continue;
            }
            used.insert(h);
            beam.push(s);
            if beam.len() >= 100000 {
                break;
            }
        }
        let live = beam
            .iter()
            .map(|s| s.id)
            .chain(vec![best_state.id])
            .collect_vec();
        let ids = trace.compact(&live);
        for s in beam.iter_mut() {
            s.id = if s.id == !0 { !0 } else { ids[s.id] };
        }
        best_state.id = if best_state.id == !0 {
            !0
        } else {
            ids[best_state.id]
        };
    }
    let b = beam.iter().min_by_key(|s| s.t).unwrap();
    for mv in trace.get(b.id) {
        println!("{}", mv);
    }
    eprintln!("Time = {:.3}", get_time());
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    visited: FixedBitSet,
    p: (i64, i64),
    v: (i64, i64),
    t: i64,
    id: usize,
}
