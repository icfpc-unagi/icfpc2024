#![allow(non_snake_case)]

use itertools::Itertools;

const TL: f64 = 60.0;

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
                    (input.ps[i].0 - state.p.0).abs() + (input.ps[i].1 - state.p.1).abs()
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
