#![allow(non_snake_case, unused)]

use num::*;
use rand::prelude::*;

const DIJ: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];
const DIR: [char; 4] = ['R', 'D', 'L', 'U'];

fn solve(input: &Input) -> String {
    let n = input.board.len();
    let m = input.board[0].len();
    let mut id = vec![!0; n * m];
    let mut vs = vec![];
    for i in 0..n {
        for j in 0..m {
            if input.board[i][j] != '#' {
                id[i * m + j] = vs.len();
                vs.push((i, j));
            }
        }
    }
    let d = vs.len();
    let mut g = mat![1000000000; d + 2; d + 2];
    for s in 0..n * m {
        if id[s] == !0 {
            continue;
        }
        let mut dist = vec![1000000000; n * m];
        let mut que = std::collections::VecDeque::new();
        dist[s] = 0;
        que.push_back(s);
        while let Some(u) = que.pop_front() {
            let d = dist[u];
            for dir in 0..4 {
                let i = u / m + DIJ[dir].0;
                let j = u % m + DIJ[dir].1;
                if i < n && j < m && input.board[i][j] != '#' && dist[i * m + j].setmin(d + 1) {
                    que.push_back(i * m + j);
                }
            }
        }
        for t in 0..n * m {
            if id[t] != !0 {
                g[id[s]][id[t]] = dist[t];
            }
        }
        g[d][id[s]] = 0;
        g[id[s]][d] = 0;
    }
    let s = id[(0..n * m)
        .find(|&s| input.board[s / m][s % m] == 'L')
        .unwrap()];
    g[s][d + 1] = 0;
    g[d + 1][s] = 0;
    g[d][d + 1] = 0;
    g[d + 1][d] = 0;
    let mut order = vec![s];
    for i in 0..d {
        if i != s {
            order.push(i);
        }
    }
    order.push(d);
    order.push(d + 1);
    order.push(s);
    order = tsp::solve(
        &g,
        &order,
        60.0,
        &mut rand_pcg::Pcg64Mcg::seed_from_u64(78436),
    );
    let mut out = vec![];
    if order[1] >= d {
        order.reverse();
    }
    for k in 0..d - 1 {
        let s = vs[order[k]];
        let t = vs[order[k + 1]];
        let mut trace = Trace::new();
        let mut dist = mat![(1000000000, !0); n; m];
        let mut que = std::collections::VecDeque::new();
        dist[s.0][s.1].0 = !0;
        que.push_back(s);
        while let Some(u) = que.pop_front() {
            let (d, id) = dist[u.0][u.1];
            if u == t {
                break;
            }
            for dir in 0..4 {
                let v = (u.0 + DIJ[dir].0, u.1 + DIJ[dir].1);
                if v.0 < n
                    && v.1 < m
                    && input.board[v.0][v.1] != '#'
                    && dist[v.0][v.1].0.setmin(d + 1)
                {
                    que.push_back(v);
                    dist[v.0][v.1].1 = trace.add(DIR[dir], id);
                }
            }
        }
        out.extend(trace.get(dist[t.0][t.1].1));
    }
    out.into_iter().collect()
}

fn main() {
    get_time();
    let input = read_input();
    let out = solve(&input);
    let id = std::env::var("INPUT").unwrap().parse::<usize>().unwrap();
    println!("{}", make_move(id, &out));
    eprintln!("Time = {:.3}", get_time());
}

fn make_move(id: usize, moves: &str) -> String {
    let mut num: BigInt = BigInt::ZERO + 1;

    for c in moves.chars() {
        num *= 4;
        match c {
            'U' => num += 0,
            'R' => num += 1,
            'D' => num += 2,
            'L' => num += 3,
            _ => {}
        }
    }

    //eprintln!("{}", moves);
    //eprintln!("move {}", num);

    let zero = "I!";
    let one = "I\"";
    // let two = "I#";
    //let three = "I$";
    let four = "I%";

    let urdl = "SOL>F";

    let y = "Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx";

    // 0: U, 1: R, 2: D, 3: L
    // let choose_char = format!("? B= B% vx {four} {zero} {su} ? B= B% vx {four} {one} {sr} ? B= B% vx {four} {two} {sd} {sl}");
    // (take 1 (drop (v2 % 4) "URDL"))
    let choose_char = format!("BT {one} BD B% vx {four} {urdl}");
    // f(x) = choose_char(x%4) . f(x/4)
    let f = format!("B$ {y} Lf Lx ? B> vx {one} B. B$ vf B/ vx {four} {choose_char} S");

    let program = format!("B$ {f} {}", encode_i(num));

    let first = format!("solve lambdaman{} ", id);
    let encoded_first = first.chars().map(encode).collect::<String>();
    let result = format!("B. S{} {}", encoded_first, program);

    result
}

fn encode_i(inp: BigInt) -> String {
    let mut i = inp;
    let zero = BigInt::from(0);
    let mut s = String::new(); // 空の文字列を初期化

    while i > zero {
        let r = (i.clone() % 94u32).to_u32().unwrap();
        s = format!("{}{}", decode_from_i(r), s);
        i /= 94u32;
    }
    format!("I{}", s)
}

fn decode_from_i(c: u32) -> char {
    // TODO: make it a constnat
    //println!("{}", c);
    //let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return (c + 33) as u8 as char;
}

fn decode(c: char) -> char {
    // TODO: make it a constnat
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars[c as usize - 33];
}

fn encode(c: char) -> char {
    // TODO: make it a constant
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    let index = chars.iter().position(|&x| x == c).unwrap();
    return (index + 33) as u8 as char;
}

// 入出力と得点計算
struct Input {
    board: Vec<Vec<char>>,
}

fn read_input() -> Input {
    let mut board = vec![];
    use std::io::prelude::*;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        board.push(line.chars().collect());
    }
    Input { board }
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
