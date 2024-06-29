#![allow(non_snake_case)]

use itertools::Itertools;
use rand::prelude::*;

const TL: f64 = 600.0;

fn get_order(input: &Input) -> Vec<usize> {
    if input.ps.len() > 10000 {
        let mut order = vec![];
        let mut visited = vec![false; input.ps.len()];
        let mut p = (0, 0);
        for _ in 0..input.ps.len() {
            let i = (0..input.ps.len())
                .filter(|&i| !visited[i])
                .min_by_key(|&i| (input.ps[i].0 - p.0).abs() + (input.ps[i].1 - p.1).abs())
                .unwrap();
            order.push(i);
            visited[i] = true;
            p = input.ps[i];
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
    let mut trace = Trace::new();
    let stime = get_time();
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(483290);
    for k in 0..input.ps.len() {
        let mut next = vec![];
        for state in beam {
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
                        for _ in 0..10000 {
                            let mut p = state.p;
                            let mut v = state.v;
                            let mut id = state.id;
                            for t in 0..T {
                                let dvx = if p.0 + v.0 * (T - t) - (T - t) * (T - t - 1) / 2
                                    <= input.ps[i].0
                                    && input.ps[i].0
                                        <= p.0 + v.0 * (T - t) + (T - t) * (T - t - 1) / 2
                                    && rng.gen_bool(0.5)
                                {
                                    0
                                } else if p.0 + v.0 * (T - t) < input.ps[i].0 {
                                    1
                                } else if p.0 + v.0 * (T - t) > input.ps[i].0 {
                                    -1
                                } else {
                                    0
                                };
                                let dvy = if p.1 + v.1 * (T - t) - (T - t) * (T - t - 1) / 2
                                    <= input.ps[i].1
                                    && input.ps[i].1
                                        <= p.1 + v.1 * (T - t) + (T - t) * (T - t - 1) / 2
                                    && rng.gen_bool(0.5)
                                {
                                    0
                                } else if p.1 + v.1 * (T - t) < input.ps[i].1 {
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
                            assert_eq!(p, input.ps[i]);
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
                T += 1;
            }
            if (get_time() - stime) > TL * (k + 1) as f64 / input.ps.len() as f64 {
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

// 入出力と得点計算
struct Input {
    ps: Vec<(i64, i64)>,
}

fn read_input() -> Input {
    use std::io::prelude::*;
    let mut ps = vec![];
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() > 0 {
            let mut it = line.split_whitespace();
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            ps.push((x, y));
        }
    }
    Input { ps }
}

// ここからライブラリ

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
	($($e:expr),*) => { vec![$($e),*] };
	($($e:expr,)*) => { vec![$($e),*] };
	($e:expr; $d:expr) => { vec![$e; $d] };
	($e:expr; $d:expr $(; $ds:expr)+) => { vec![mat![$e $(; $ds)*]; $d] };
}

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        // ローカル環境とジャッジ環境の実行速度差はget_timeで吸収しておくと便利
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.0
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

pub struct Trace<T: Copy> {
    log: Vec<(T, usize)>,
}

impl<T: Copy> Trace<T> {
    pub fn new() -> Self {
        Trace { log: vec![] }
    }
    pub fn add(&mut self, c: T, p: usize) -> usize {
        self.log.push((c, p));
        self.log.len() - 1
    }
    pub fn get(&self, mut i: usize) -> Vec<T> {
        let mut out = vec![];
        while i != !0 {
            out.push(self.log[i].0);
            i = self.log[i].1;
        }
        out.reverse();
        out
    }
    pub fn len(&self) -> usize {
        self.log.len()
    }
    pub fn compact(&mut self, live: &[usize]) -> Vec<usize> {
        let mut new_id = vec![!0; self.log.len()];
        for &i in live {
            if i != !0 {
                new_id[i] = 0;
            }
        }
        for i in (0..self.log.len()).rev() {
            if new_id[i] == 0 && self.log[i].1 != !0 {
                new_id[self.log[i].1] = 0;
            }
        }
        let mut n = 0;
        for i in 0..self.log.len() {
            if new_id[i] == 0 {
                new_id[i] = n;
                n += 1;
            }
        }
        let mut log = Vec::with_capacity(n);
        for i in 0..self.log.len() {
            if new_id[i] != !0 {
                log.push((
                    self.log[i].0,
                    if self.log[i].1 == !0 {
                        !0
                    } else {
                        new_id[self.log[i].1]
                    },
                ));
            }
        }
        self.log = log;
        new_id
    }
}

use rand::SeedableRng;
use rustc_hash::FxHashSet;
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
struct Entry<K, V> {
    k: K,
    v: V,
}

impl<K: PartialOrd, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<K: PartialOrd, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.k.partial_cmp(&other.k)
    }
}

impl<K: PartialEq, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.k.eq(&other.k)
    }
}

impl<K: PartialEq, V> Eq for Entry<K, V> {}

/// K が小さいトップn個を保持
#[derive(Clone, Debug)]
pub struct BoundedSortedList<K: PartialOrd + Copy, V: Clone> {
    que: BinaryHeap<Entry<K, V>>,
    size: usize,
}

impl<K: PartialOrd + Copy, V: Clone> BoundedSortedList<K, V> {
    pub fn new(size: usize) -> Self {
        Self {
            que: BinaryHeap::with_capacity(size),
            size,
        }
    }
    pub fn can_insert(&self, k: K) -> bool {
        self.que.len() < self.size || self.que.peek().unwrap().k > k
    }
    pub fn insert(&mut self, k: K, v: V) {
        if self.que.len() < self.size {
            self.que.push(Entry { k, v });
        } else if let Some(mut top) = self.que.peek_mut() {
            if top.k > k {
                top.k = k;
                top.v = v;
            }
        }
    }
    pub fn list(&self) -> Vec<(K, V)> {
        let v = self.que.clone().into_sorted_vec();
        v.into_iter().map(|e| (e.k, e.v)).collect()
    }
    pub fn len(&self) -> usize {
        self.que.len()
    }
}

mod tsp {

    use super::*;
    use rand_pcg::Pcg64Mcg;
    type C = i64;

    pub fn compute_cost(g: &Vec<Vec<C>>, ps: &Vec<usize>) -> C {
        let mut tmp = 0;
        for i in 0..ps.len() - 1 {
            tmp += g[ps[i]][ps[i + 1]];
        }
        tmp
    }

    // mv: (i, dir)
    pub fn apply_move(tour: &mut Vec<usize>, idx: &mut Vec<usize>, mv: &[(usize, usize)]) {
        let k = mv.len();
        let mut ids: Vec<_> = (0..k).collect();
        ids.sort_by_key(|&i| mv[i].0);
        let mut order = vec![0; k];
        for i in 0..k {
            order[ids[i]] = i;
        }
        let mut tour2 = Vec::with_capacity(mv[ids[k - 1]].0 - mv[ids[0]].0);
        let mut i = ids[0];
        let mut dir = 0;
        loop {
            let (j, rev) = if dir == mv[i].1 {
                ((i + 1) % k, 0)
            } else {
                ((i + k - 1) % k, 1)
            };
            if mv[j].1 == rev {
                if order[j] == k - 1 {
                    break;
                } else {
                    i = ids[order[j] + 1];
                    dir = 0;
                    tour2.extend_from_slice(&tour[mv[j].0 + 1..mv[i].0 + 1]);
                }
            } else {
                i = ids[order[j] - 1];
                dir = 1;
                tour2.extend(tour[mv[i].0 + 1..mv[j].0 + 1].iter().rev().cloned());
            }
        }
        assert_eq!(tour2.len(), mv[ids[k - 1]].0 - mv[ids[0]].0);
        tour[mv[ids[0]].0 + 1..mv[ids[k - 1]].0 + 1].copy_from_slice(&tour2);
        for i in mv[ids[0]].0 + 1..mv[ids[k - 1]].0 + 1 {
            idx[tour[i]] = i;
        }
    }

    pub const FEASIBLE3: [bool; 64] = [
        false, false, false, true, false, true, true, true, true, true, true, false, true, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        true, false, true, true, true, true, true, true, false, true, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, true, false, true,
        true, true, true, true, true, false, true, false, false, false,
    ];

    pub fn solve(g: &Vec<Vec<C>>, qs: &Vec<usize>, until: f64, rng: &mut Pcg64Mcg) -> Vec<usize> {
        let n = g.len();
        let mut f = vec![vec![]; n];
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    f[i].push((g[i][j], j));
                }
            }
            f[i].sort_by(|&(a, _), &(b, _)| a.partial_cmp(&b).unwrap());
        }
        let mut ps = qs.clone();
        let mut idx = vec![!0; n];
        let (mut min, mut min_ps) = (compute_cost(&g, &qs), ps.clone());
        while get_time() < until {
            let mut cost = compute_cost(&g, &ps);
            for p in 0..n {
                idx[ps[p]] = p;
            }
            loop {
                let mut ok = false;
                for i in 0..n {
                    for di in 0..2 {
                        'loop_ij: for &(ij, vj) in &f[ps[i + di]] {
                            if g[ps[i]][ps[i + 1]] - ij <= 0 {
                                break;
                            }
                            for dj in 0..2 {
                                let j = if idx[vj] == 0 && dj == 0 {
                                    n - 1
                                } else {
                                    idx[vj] - 1 + dj
                                };
                                let gain = g[ps[i]][ps[i + 1]] - ij + g[ps[j]][ps[j + 1]];
                                // 2-opt
                                if di != dj && gain - g[ps[j + dj]][ps[i + 1 - di]] > 0 {
                                    cost -= gain - g[ps[j + dj]][ps[i + 1 - di]];
                                    apply_move(&mut ps, &mut idx, &[(i, di), (j, dj)]);
                                    ok = true;
                                    break 'loop_ij;
                                }
                                // 3-opt
                                for &(jk, vk) in &f[ps[j + dj]] {
                                    if gain - jk <= 0 {
                                        break;
                                    }
                                    for dk in 0..2 {
                                        let k = if idx[vk] == 0 && dk == 0 {
                                            n - 1
                                        } else {
                                            idx[vk] - 1 + dk
                                        };
                                        if i == k || j == k {
                                            continue;
                                        }
                                        let gain = gain - jk + g[ps[k]][ps[k + 1]];
                                        if gain - g[ps[k + dk]][ps[i + 1 - di]] > 0 {
                                            let mask = if i < j { 1 << 5 } else { 0 }
                                                | if i < k { 1 << 4 } else { 0 }
                                                | if j < k { 1 << 3 } else { 0 }
                                                | di << 2
                                                | dj << 1
                                                | dk;
                                            if FEASIBLE3[mask] {
                                                cost -= gain - g[ps[k + dk]][ps[i + 1 - di]];
                                                apply_move(
                                                    &mut ps,
                                                    &mut idx,
                                                    &[(i, di), (j, dj), (k, dk)],
                                                );
                                                ok = true;
                                                break 'loop_ij;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if !ok {
                    break;
                }
            }
            if min.setmin(cost) {
                min_ps = ps;
                eprintln!("{:.3}: {}", get_time(), min);
            }
            ps = min_ps.clone();
            if n <= 4 {
                break;
            }
            loop {
                if rng.gen_range(0..2) == 0 {
                    // double bridge
                    let mut is: Vec<_> = (0..4).map(|_| rng.gen_range(0..n)).collect();
                    is.sort();
                    if is[0] == is[1] || is[1] == is[2] || is[2] == is[3] {
                        continue;
                    }
                    ps = ps[0..is[0] + 1]
                        .iter()
                        .chain(ps[is[2] + 1..is[3] + 1].iter())
                        .chain(ps[is[1] + 1..is[2] + 1].iter())
                        .chain(ps[is[0] + 1..is[1] + 1].iter())
                        .chain(ps[is[3] + 1..].iter())
                        .cloned()
                        .collect();
                } else {
                    for _ in 0..6 {
                        loop {
                            let i = rng.gen_range(1..n);
                            let j = rng.gen_range(1..n);
                            if i < j && j - i < n - 2 {
                                ps = ps[0..i]
                                    .iter()
                                    .chain(ps[i..j + 1].iter().rev())
                                    .chain(ps[j + 1..].iter())
                                    .cloned()
                                    .collect();
                                break;
                            }
                        }
                    }
                }
                break;
            }
        }
        min_ps
    }
}
