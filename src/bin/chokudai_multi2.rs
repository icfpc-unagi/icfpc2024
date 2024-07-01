#![allow(unused)]

extern crate num_bigint;
extern crate num_traits;

use core::num;
use itertools::Itertools;
use itertools::{KMerge, KMergeBy};
use num::*;
use num_bigint::{BigInt, ToBigInt};
use num_traits::{PrimInt, ToPrimitive};
use rand::prelude::*;
use rayon::{array, prelude::*, string};
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

fn solve2(input: &Input, step: i32, xnum: usize, snum: usize, steps: usize) -> i32 {
    let step = steps;
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

    let best_result = Mutex::new((9999999, 99999999)); // (solve(i), i)

    let mut base_array = vec![];
    for i in 0..snum {
        base_array.push(0);
        base_array.push(1);
        base_array.push(2);
        base_array.push(3);
    }
    base_array.push(5);
    base_array.push(5);
    for i in 0..xnum {
        base_array.push(4);
    }

    let max = count_permutations(&base_array) as i64;

    loop {
        let range = 0..max;
        range.into_par_iter().for_each(|i| {
            if best_result.lock().unwrap().0 == 0 {
                return;
            }
            let ii = i;

            let result = solve3(ii, start_id, d, xnum, snum, step, &next);

            // もしsolve(i)が0ならば即終了
            if result.0 == 0 {
                let mut best = best_result.lock().unwrap();
                *best = (result.0, ii);
                println!("found!");
                return;
            }

            // 最小値を更新
            let mut best = best_result.lock().unwrap();
            if result.0 < best.0 {
                *best = (result.0, ii);

                println!("  NowBest: {} at i: {}", best.0, best.1);
            }
        });
        break;
    }

    let best_result = best_result.lock().unwrap();
    if best_result.0 == 0 {
        println!("OK : {}", best_result.1);
        return 1;
    } else {
        println!("NG {} at i: {}", best_result.0, best_result.1);
    }

    return 0;
}

fn solve3(
    bit: i64,
    start_id: usize,
    d: usize,
    xnum: usize,
    snum: usize,
    step: usize,
    next: &Vec<Vec<usize>>,
) -> (usize, usize) {
    let mut visited = vec![false; d];
    let mut loopflag = vec![false; d];
    visited[start_id] = true;
    let mut now = start_id;
    let mut remain_pos = d - 1;

    let perm_length = xnum + snum * 4 + 2;
    let mut perm = vec![0; perm_length];

    let mut remain_bit = bit;
    for i in 0..perm_length {
        perm[i] = (remain_bit % (i + 1) as i64) as usize;
        remain_bit /= (i + 2) as i64;
    }

    for k in 0..perm.len() {
        for i in 0..k {
            if perm[i] >= perm[k] {
                perm[i] += 1;
            }
        }
    }

    let mut error = false;
    let mut xmin = 0;
    let mut comin = xnum;

    let mut amin = vec![];
    for i in 0..4 {
        amin.push(i + xnum + 2);
    }

    let mut base_array = vec![0; perm_length];
    let mut array_start = !0;
    let mut subarray_start = !0;

    for i in 0..perm_length {
        if perm[i] < xnum {
            if array_start == !0 && perm[i] != xmin {
                error = true;
                break;
            }
            xmin += 1;

            base_array[i] = 4;
        } else if perm[i] < xnum + 2 {
            if subarray_start == !0 {
                subarray_start = i;
                if xnum != perm[i] {
                    error = true;
                    break;
                }
            } else {
                array_start = i;
            }
            base_array[i] = 5;
        } else {
            let rem = perm[i] - (xnum + 2);
            let r = rem % 4;
            if amin[r] != perm[i] {
                error = true;
                break;
            }
            amin[r] += 4;

            base_array[i] = r;
        }
    }
    //eprintln!("OK ar:{} sub:{}", array_start, subarray_start);

    if error {
        return (remain_pos, 0);
    }

    let limit_turn = 999998 / step;

    let mut now_array = vec![];
    let mut now_size = 0;

    for i in 0..subarray_start {
        for k in 0..step {
            now_array.push(base_array[i]);
        }
    }
    now_size = now_array.len();

    let mut afterxnum = xnum;
    let mut aftersnum = 0;
    for i in array_start..base_array.len() {
        if base_array[i] < 4 {
            aftersnum += 1;
        }
    }

    let mut first_flag = true;

    while now_size * xnum + step * aftersnum < 1000000 {
        let mut next_array = vec![];

        if first_flag {
            first_flag = false;
            for k in subarray_start + 1..array_start {
                for i in 0..step {
                    next_array.push(base_array[k]);
                }
            }
        }

        for i in array_start + 1..base_array.len() {
            if base_array[i] < 4 {
                for k in 0..step {
                    next_array.push(base_array[i]);
                }
            } else {
                for k in 0..now_array.len() {
                    next_array.push(now_array[k]);
                }
            }
        }

        now_array = next_array;
        now_size = now_array.len();
    }

    for turn in 0..now_size {
        now = next[now][now_array[turn]];

        if !visited[now] {
            remain_pos -= 1;
            visited[now] = true;
            if remain_pos == 0 {
                println!("OK {} {}", remain_pos, turn);
                let mut s = String::new();
                for i in 0..base_array.len() {
                    if base_array[i] < 4 {
                        s.push(DIR[base_array[i]]);
                    } else if base_array[i] == 4 {
                        s.push('X');
                    } else {
                        s.push(',');
                    }
                }
                println!("{} {}", s, turn);

                return (0, turn);
            }
        }
    }
    return (remain_pos, 0);
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn count_permutations(arr: &[usize]) -> usize {
    let mut freq = HashMap::new();
    for &num in arr {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut denom = 1;
    for &count in freq.values() {
        denom *= factorial(count);
    }

    factorial(arr.len()) / denom
}

// 指定したインデックスのユニークなpermutationを返す関数
fn get_unique_permutation(mut arr: Vec<usize>, mut index: usize) -> Option<Vec<usize>> {
    arr.sort();
    let mut result = Vec::new();
    let n = arr.len();
    let mut factorials = vec![1; n + 1];
    for i in 1..=n {
        factorials[i] = factorials[i - 1] * i;
    }

    while !arr.is_empty() {
        let mut freq = HashMap::new();
        for &num in &arr {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut cum_freq = 0;
        for num in freq.keys().cloned().collect::<Vec<_>>() {
            let count = freq[&num];
            let perm_count = factorials[arr.len() - 1] / factorials[count - 1];
            if cum_freq + perm_count > index {
                result.push(num);
                arr.retain(|&x| x != num);
                index -= cum_freq;
                break;
            }
            cum_freq += perm_count;
        }
    }

    Some(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let id: usize = args[1].parse().expect("ID should be an integer");
    let xnum: usize = args[2].parse().expect("Step should be an integer");
    let snum: usize = args[3].parse().expect("Step should be an integer");
    let step: usize = args[4].parse().expect("Step should be an integer");

    solve(id, step, xnum, snum, step);
}

fn solve(i: usize, step: usize, xnum: usize, snum: usize, steps: usize) {
    let filename = format!("input/lambdaman/lambdaman{}.txt", i);
    let input = read_input_from_file(filename);
    _ = get_time(true);

    eprintln!("Test case {}", i);
    let moves = solve2(&input, step as i32, xnum, snum, steps);
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
use std::collections::{BinaryHeap, HashMap};

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
