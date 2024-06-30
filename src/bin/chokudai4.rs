#![allow(unused)]

extern crate num_bigint;
extern crate num_traits;

use core::num;
use itertools::{KMerge, KMergeBy};
use num::*;
use num_bigint::BigInt;
use num_traits::{PrimInt, ToPrimitive};
use rand::prelude::*;
use rayon::prelude::*;
use resvg::usvg::filter::Turbulence;
use std::char::MAX;
use std::env;
use std::f32::consts::E;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::path::{self, Path};
use std::sync::Mutex;
use std::thread::{self, sleep};
use std::time::Duration;
use tokio::sync::mpsc::error::SendError;

use icfpc2024::{communicate, eval};

type Board = Vec<Vec<char>>;

const DIJ: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];
const DIR: [char; 4] = ['R', 'D', 'L', 'U'];

fn solve2(input: &Input, step: i32, first_mod: usize) -> i32 {
    let n = input.board.len();
    let m = input.board[0].len();
    let mut id = vec![!0; n * m];
    let mut vs = vec![];
    let mut start_id = !0;
    for i in 0..n {
        for j in 0..m {
            if input.board[i][j] != '#' {
                id[i * m + j] = vs.len();
                if input.board[i][j] == 'L' {
                    start_id = vs.len();
                }
                vs.push((i, j));
            }
        }
    }
    let mut next = vec![vec![!0; 4]; vs.len()];

    let d = vs.len();
    for s in 0..(n * m) {
        if id[s] == !0 {
            continue;
        }
        let (i, j) = vs[id[s]];
        for k in 0..4 {
            let (i2, j2) = (i.wrapping_add(DIJ[k].0), j.wrapping_add(DIJ[k].1));
            if i2 < n && j2 < m && input.board[i2][j2] != '#' {
                next[id[s]][k] = id[i2 * m + j2];
            } else {
                next[id[s]][k] = id[s];
            }
        }
    }

    //let prime = vec![1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    //let range = 0..=1 << (2 * prime.len());

    // 並列処理の結果を保存するためのMutexを用意します。
    let best_result = Mutex::new((9999999, 99999999)); // (solve(i), i)
    let challenge = Mutex::new(1); //

    let mut challenge = 0;

    let mut modulo = first_mod;

    loop {
        eprintln!(
            "now modulo: {} NowBest: {}",
            modulo,
            best_result.lock().unwrap().0
        );

        let range = 0..(93 * 93);
        range.into_par_iter().for_each(|i| {
            if best_result.lock().unwrap().0 == 0 {
                return;
            }
            let ii = i;

            let result = solve3(ii, start_id, d, step as usize, &next, modulo);

            // もしsolve(i)が0ならば即終了
            if result.0 == 0 {
                let last_turn = result.1;
                let a = ii / 93 % 93 + 1;
                let b = ii % 93 + 1;
                let last = getLastA(a, b, step as usize, modulo, last_turn);
                if last[3] == !0 {
                    eprintln!("endpoint is not found");
                    return;
                }

                let mut best = best_result.lock().unwrap();
                *best = (result.0, ii);
                println!("found!");
                return;
            }

            challenge.add(1);

            // 最小値を更新
            let mut best = best_result.lock().unwrap();
            if result.0 < best.0 {
                *best = (result.0, ii);

                println!("  NowBest: {} at i: {} {}", best.0, best.1, challenge);
            }
        });

        if best_result.lock().unwrap().0 == 0 {
            break;
        } else {
            loop {
                modulo += 1;
                let mut is_prime = true;
                for div in 2..modulo {
                    if modulo % div == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    break;
                }
            }
        }
    }

    let best_result = best_result.lock().unwrap();
    if best_result.0 == 0 {
        println!("OK : {}", best_result.1);
        let a = best_result.1 / 93 % 93 + 1;
        let b = best_result.1 % 93 + 1;
        println!("a : {}", a);
        println!("b : {}", b);
        println!("mod : {}", modulo);
        let last = getLastA(a, b, step as usize, modulo, 1);
        println!("last : {} {} {} {}", last[0], last[1], last[2], last[3]);
        /*
        for p in 0..prime.len() {
            let mul = (best_result.1 as i32 >> (p * 2)) % 4;
            if mul != 0 {
                let pp = prime[p];
                println!("      {pp} {mul}");
            }
        }
        */
        return 1;
    } else {
        println!("NG {} at i: {}", best_result.0, best_result.1);
    }

    return 0;
}

fn solve3(
    bit: usize,
    start_id: usize,
    d: usize,
    step: usize,
    next: &Vec<Vec<usize>>,
    modulo: usize,
) -> (usize, usize) {
    //let prime = vec![1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let mut visited = vec![false; d];
    let mut loopflag = vec![false; d];
    visited[start_id] = true;
    let mut now = start_id;
    let mut remain_pos = d - 1;

    /*
    let mut loop_length = 1;
    let mut prime_count = 0;

    for p in 0..prime.len() {
        let mul = (bit as i32 >> (p * 2)) % 4;
        if mul != 0 {
            loop_length *= prime.len();
            prime_count += 1;
        }
    }
    */

    let limit_turn = 999998 / step;

    let b = bit % 93 + 1;
    let mut a = bit / 93 % 93 + 1;
    if b <= 1 {
        return (remain_pos, 0);
    }

    for turn in 0..limit_turn {
        let rt = limit_turn - turn;

        /*
        if turn as usize % loop_length == 0 {
            if loopflag[now] {
                break;
            }
            loopflag[now] = true;
        }
        */

        /*
        let mut r = 0;
        for p in 0..prime.len() {
            let mul = ((bit as i32 >> (p * 2)) % 4) as usize;
            r += (rt / prime[p]) * mul % 4;
        }
        r %= 4;
        */
        let r = a % 4;

        a = (a * b) % modulo;

        for k in 0..step {
            now = next[now][r as usize];

            if !visited[now] {
                remain_pos -= 1;
                visited[now] = true;
                if remain_pos == 0 {
                    /*
                    println!("  OK {} {} {}", step, remain_pos, turn + 1);
                    for p in 0..prime.len() {
                        let mul = (bit as i32 >> (p * 2)) % 4;
                        if mul != 0 {
                            let pp = prime[p];
                            println!("      {pp} {mul}");
                        }
                    }
                    */
                    return (0, turn);
                }
            }
        }
    }
    return (remain_pos, 0);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let id: usize = args[1].parse().expect("ID should be an integer");
    let step: usize = args[2].parse().expect("Step should be an integer");
    let modulo: usize = if args.len() > 3 {
        args[3].parse().expect("Modulo should be an integer")
    } else {
        1000003
    };

    solve(id, step, modulo);
}

fn solve(i: usize, step: usize, first_mod: usize) {
    let filename = format!("input/lambdaman/lambdaman{}.txt", i);
    let input = read_input_from_file(filename);
    _ = get_time(true);

    eprintln!("Test case {}", i);
    let moves = solve2(&input, step as i32, first_mod);
}

fn decode(c: char) -> char {
    // TODO: make it a constnat
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars[c as usize - 33];
}

fn echoeval(input: &str) -> anyhow::Result<String> {
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw request:\n{}\n", &input);

    let body = communicate(r"B. S%#(/} ".to_string() + input)?;

    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw response:\n{}\n", body);

    let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Decoded response:\n{}\n", decoded_text);

    let suffix = "\nYou scored some points for using the echo service!\n";
    assert!(decoded_text.ends_with(suffix));
    let decoded_text = decoded_text[..decoded_text.len() - suffix.len()].to_owned();

    Ok(decoded_text)
}

fn request(input: &str) -> anyhow::Result<String> {
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw request:\n{}\n", &input);

    let text = input;
    //let text = "S".to_owned() + &input.chars().map(encode).collect::<String>();

    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Encoded request:\n{}\n", &text);

    let body = communicate(text.to_string())?;

    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Raw response:\n{}\n", body);

    if body.starts_with("B") {
        thread::sleep(Duration::from_secs(3));
        echoeval(&body)
    } else {
        let decoded_text = body.chars().skip(1).map(decode).collect::<String>();
        eprintln!(
            "--------------------------------------------------------------------------------"
        );
        eprintln!("Decoded response:\n{}\n", decoded_text);
        Ok(decoded_text)
    }
}

fn getLastA(a: usize, b: usize, step: usize, modulo: usize, end_turn: usize) -> Vec<usize> {
    let mut visited = vec![false; modulo];

    let mut ans = vec![!0; 4];
    let max_turn = 999998 / step;
    let mut a2 = a;
    for i in 0..max_turn {
        if !visited[a2] && i >= end_turn {
            if a2 < 94 {
                ans[0] = a2;
            }
            if a2 < 94 * 94 {
                ans[1] = a2;
            }
            if a2 < 94 * 94 * 94 {
                ans[2] = a2;
            }
            if a2 < 94 * 94 * 94 * 94 {
                ans[3] = a2;
            }
        }
        visited[a2] = true;
        a2 = ((a2 as u64 * b as u64) % modulo as u64) as usize;
    }

    return ans;
}
struct Input {
    board: Vec<Vec<char>>,
}

fn read_input_from_file(filename: String) -> Input {
    let mut board = vec![];

    // ファイルを開く
    let file = File::open(filename).expect("ファイルを開けませんでした");
    let reader = io::BufReader::new(file);

    // ファイルから1行ずつ読み込む
    for line in reader.lines() {
        let line = line.expect("行を読み込めませんでした");
        let line = line.trim();
        board.push(line.chars().collect());
    }

    Input { board }
}

fn read_input() -> Input {
    let mut board = vec![];
    use std::io::prelude::*;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        board.push(line.chars().collect());
    }
    let flag = false;

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

pub fn get_time(b: bool) -> f64 {
    static mut STIME: f64 = -1.0;
    if b {
        unsafe {
            STIME = -1.0;
        }
    }
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
