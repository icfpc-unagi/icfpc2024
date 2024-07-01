#![allow(non_snake_case)]

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use solution::*;

const TL: f64 = 1.0 * 3600.0;
const DT: i64 = 10;
const TARGET_T: i64 = 1;

// dt秒後にちょうどdだけ進むための加速方法と終速度を列挙
fn listup(dt: i64, d: i64) -> Vec<(i64, Vec<i64>)> {
    if dt > 50 {
        return vec![find_acc(dt, d)];
    }
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

// dt秒後にちょうどdだけ進むための加速方法を一つ求める
fn find_acc(dt: i64, mut d: i64) -> (i64, Vec<i64>) {
    let mut out = vec![];
    let mut v = 0;
    for t in 0..dt {
        let cand = (-1..=1)
            .filter(|&a| {
                (v + a) * (dt - t) - (dt - t) * (dt - t - 1) / 2 <= d
                    && d <= (v + a) * (dt - t) + (dt - t) * (dt - t - 1) / 2
            })
            .collect_vec();
        let a = if v > 0 {
            cand[cand.len() - 1]
        } else if v < 0 {
            cand[0]
        } else {
            cand[cand.len() / 2]
        };
        v += a;
        d -= v;
        out.push(a);
    }
    (v, out)
}

fn main() {
    get_time();
    if input_id() < 18 {
        return;
    }
    let mut input = read_input();
    input.ps.sort();
    input.ps.dedup();
    let mut trace = Trace::new();
    let mut beam = vec![State {
        t: 0,
        p: (0, 0),
        v: (0, 0),
        visited: FixedBitSet::with_capacity(input.ps.len()),
        id: !0,
    }];
    let best = if let Ok(best) = std::env::var("BEST") {
        read_output(&best)
    } else {
        vec![]
    };
    if best.len() == input.ps.len() {
        return;
    }
    let mut best_state = State {
        t: 0,
        p: (0, 0),
        v: (0, 0),
        visited: FixedBitSet::with_capacity(input.ps.len()),
        id: !0,
    };
    let mut cache_vs = FxHashMap::default();
    for k in 0..input.ps.len() {
        let mut list = vec![];
        let tl = get_time() + TL as f64 / input.ps.len() as f64 * 0.5;
        for s in 0..beam.len() {
            let p = beam[s].p;
            let v = beam[s].v;
            let target = {
                let mut list = BoundedSortedList::new(10);
                for i in 0..input.ps.len() {
                    if !beam[s].visited[i] {
                        list.insert(
                            (input.ps[i].0 - p.0 - v.0 * TARGET_T).abs()
                                + (input.ps[i].1 - p.1 - v.1 * TARGET_T).abs(),
                            i,
                        );
                    }
                }
                list.list().into_iter().map(|(_, i)| i).collect_vec()
            };
            for i in target {
                let dx = input.ps[i].0 - p.0;
                let dy = input.ps[i].1 - p.1;
                let mut T = 0;
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
                            list.push((beam[s].t + T, s, i));
                        }
                    }
                }
            }
            if get_time() > tl {
                break;
            }
        }
        list.sort();
        let mut next = vec![];
        let mut used = FxHashSet::default();
        let tl = get_time() + TL as f64 / input.ps.len() as f64 * 0.5;
        'lp: for (T, s, i) in list {
            let q = input.ps[i];
            let dx = q.0 - beam[s].p.0;
            let dy = q.1 - beam[s].p.1;
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
            let cvx = &cache_vs[&(T - beam[s].t, dx - beam[s].v.0 * (T - beam[s].t))];
            let cvy = &cache_vs[&(T - beam[s].t, dy - beam[s].v.1 * (T - beam[s].t))];
            // 速度を最小・最大・絶対値最小のみに絞る
            let mut vx = vec![&cvx[0]];
            if cvx.len() >= 2 {
                vx.push(&cvx[cvx.len() - 1]);
                let p = (0..cvx.len())
                    .min_by_key(|&i| (cvx[i].0 + beam[s].v.0).abs())
                    .unwrap();
                if p > 0 && p + 1 < cvx.len() {
                    vx.push(&cvx[p]);
                }
            }
            let mut vy = vec![&cvy[0]];
            if cvy.len() >= 2 {
                vy.push(&cvy[cvy.len() - 1]);
                let p = (0..cvy.len())
                    .min_by_key(|&i| (cvy[i].0 + beam[s].v.1).abs())
                    .unwrap();
                if p > 0 && p + 1 < cvy.len() {
                    vy.push(&cvy[p]);
                }
            }
            for (vx, ax) in vx.iter() {
                for (vy, ay) in vy.iter() {
                    let vx = beam[s].v.0 + *vx;
                    let vy = beam[s].v.1 + *vy;
                    let mut visited = beam[s].visited.clone();
                    visited.insert(i);
                    let h = (visited.clone(), i, vx, vy);
                    if used.insert(h) {
                        let mut id = beam[s].id;
                        for i in 0..ax.len() {
                            id = trace.add((ay[i] + 1) * 3 + (ax[i] + 1) + 1, id);
                        }
                        next.push(State {
                            t: T,
                            p: input.ps[i],
                            v: (vx, vy),
                            visited,
                            id,
                        });
                        if next.len() > 1000000 {
                            break 'lp;
                        }
                    }
                }
            }
            if get_time() > tl {
                break;
            }
        }
        if cache_vs.len() > 1000000 {
            cache_vs.clear();
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
    let out = trace.get(beam[0].id);
    for mv in out {
        println!("{}", mv);
    }
    eprintln!("Time = {:.3}", get_time());
}

#[derive(Clone, Debug)]
struct State {
    t: i64,
    visited: FixedBitSet,
    p: (i64, i64),
    v: (i64, i64),
    id: usize,
}
