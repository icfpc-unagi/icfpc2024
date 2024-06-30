#![allow(unused)]

extern crate num_bigint;
extern crate num_traits;
use clap::Parser;

use core::num;
use itertools::{KMerge, KMergeBy};
use num::*;
use num_bigint::BigInt;
use num_traits::{PrimInt, ToPrimitive};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
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

fn gen(problem_id: i64, x0: i64, a: i64, c: i64, m: i64, xt: i64, step: i32) -> String {
    if step == 1 {
        if c != 0 {
            return format!(
                r##"
        B.
            "solve lambdaman{problem_id} "
            B$
                B$
                    Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
                    Lf Lx
                    ?
                        B= vx {xt}
                        ""
                        B.
                            BT 1 BD B% vx 4 "RDLU"
                            B$ vf
                                B%
                                    B+
                                        B*
                                            vx
                                            {a}
                                        {c}
                                    {m}
            {x0}
        "##
            );
        } else {
            return format!(
                r##"
        B.
            "solve lambdaman{problem_id} "
            B$
                B$
                    Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
                    Lf Lx
                    ?
                        B= vx {xt}
                        ""
                        B.
                            BT 1 BD B% vx 4 "RDLU"
                            B$ vf
                                B%
                                    B*
                                        vx
                                        {a}
                                    {m}
            {x0}
        "##
            );
        }
    } else if step == 2 {
        if c != 0 {
            return format!(
                r##"
        B.
            "solve lambdaman{problem_id} "
            B$
                B$
                    Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
                    Lf Lx
                    ?
                        B= vx {xt}
                        ""
                        B.
                            BT 2 BD B% B* vx 2 8 "RRDDLLUU"
                            B$ vf
                                B%
                                    B+
                                        B*
                                            vx
                                            {a}
                                        {c}
                                    {m}
            {x0}
        "##
            );
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

fn solve2(problem_id: usize, input: &Input, step: i32, first_mod: usize, use_c: bool) -> i32 {
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
    let best_result = Mutex::new((9999999, !(0 as usize), !(0 as usize), !(0 as usize))); // (solve(i), i)
    let challenge = Mutex::new(1); //

    let mut challenge = 0;

    let mut modulo = first_mod;
    let mut rng = thread_rng();

    loop {
        eprintln!("now modulo: {}", modulo);

        //let range = 0..(93 * 93 * 93);

        let mut range: Vec<u64>;
        if use_c {
            range = (0..(93 * 93 * 93)).collect();
        } else {
            range = vec![];
            for a in 0..93 {
                for b in 0..93 {
                    range.push((a * 93 + b) * 93);
                }
            }
        }
        range.shuffle(&mut rng);
        // let range: Vec<_> = range.iter().take(1000).collect();

        range.into_par_iter().for_each(|i| {
            if best_result.lock().unwrap().0 == 0 {
                return;
            }
            // let ii = ((i as u64 * 123456711) % (93 * 93 * 93)) as usize;
            let ii = i as usize;
            let a = ii / 93 / 93 + 1;
            let b = ii / 93 % 93 + 1;
            let c = if use_c { ii % 93 + 1 } else { 0 };

            let (remain_pos, last_turn) =
                solve3(a, b, c, start_id, d, step as usize, &next, modulo);

            // 最小値を更新
            let mut best = best_result.lock().unwrap();
            if remain_pos < best.0 {
                *best = (remain_pos, a, b, c);
                println!("  NowBest: {} at i: {} {}", best.0, best.1, challenge);
            }

            // もしsolve(i)が0ならば即終了 → そう甘くない！lastの0が0になるまで終了しない
            if remain_pos == 0 {
                let last = getLastA(a, b, c, step as usize, modulo, last_turn);

                for i in 0..last.len() {
                    if last[i] == !0 {
                        continue;
                    }
                    let program = gen(
                        problem_id as i64,
                        a as i64,
                        b as i64,
                        c as i64,
                        modulo as i64,
                        last[i] as i64,
                        step,
                    );
                    eprintln!("--------------------------------------------------------------------------------");
                    println!("a : {}", a);
                    println!("b : {}", b);
                    println!("c : {}", c);
                    println!("mod : {}", modulo);
                    println!("last : {}", last[i]);

                    eprintln!("Program:\n{}", program);
                    let compiled = icfpc2024::pp::preprocess(&program).unwrap();
                    eprintln!("Compiled:\n{}\n(length={})", compiled, compiled.len());
                    break;
                }

                if last[0] == !0 {
                    eprintln!("endpoint is not found");
                    return;
                }

                let mut best = best_result.lock().unwrap();
                *best = (remain_pos, a, b, c);
                println!("found!");
                return;
            }

            challenge.add(1);

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
        let a = best_result.1;
        let b = best_result.2;
        let c = best_result.3;
        println!("a : {}", a);
        println!("b : {}", b);
        println!("c : {}", c);
        println!("mod : {}", modulo);
        let last = getLastA(a, b, c, step as usize, modulo, 1);
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

        /*
        for last in last {
            let program = gen1(
                problem_id as i64,
                a as i64,
                b as i64,
                c as i64,
                modulo as i64,
                last as i64,
            );
            eprintln!("{}", program);
            let compiled = icfpc2024::pp::preprocess(&program).unwrap();
            eprintln!("{}", compiled);
        }
        */

        return 1;
    } else {
        println!("NG {} at i: {}", best_result.0, best_result.1);
    }

    return 0;
}

fn solve3(
    mut a: usize,
    b: usize,
    c: usize,
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

    if a <= 1 {
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

        //let r = a % (4 * step);
        let r = a % 4;
        a = (a * b + c) % modulo;

        for k in 0..step {
            //now = next[now][(r + k) / step % 4 as usize];
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

/// A simple program to send file contents as requests
#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    problem: usize,

    #[arg(long)]
    step: usize,

    #[arg(long)]
    modulo: usize,

    #[arg(long)]
    use_c: bool,
}

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    let id: usize = args[1].parse().expect("ID should be an integer");
    let step: usize = args[2].parse().expect("Step should be an integer");
    let modulo: usize = if args.len() > 3 {
        args[3].parse().expect("Modulo should be an integer")
    } else {
        1000003
    };
    */
    let args = Args::parse();
    dbg!(&args);

    solve(args.problem, args.step, args.modulo, args.use_c);
}

fn solve(i: usize, step: usize, first_mod: usize, use_c: bool) {
    let filename = format!("input/lambdaman/lambdaman{}.txt", i);
    let input = read_input_from_file(filename);
    _ = get_time(true);

    eprintln!("Test case {}", i);
    let moves = solve2(i, &input, step as i32, first_mod, use_c);
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

fn getLastA(
    a: usize,
    b: usize,
    c: usize,
    step: usize,
    modulo: usize,
    end_turn: usize,
) -> Vec<usize> {
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
        a2 = ((a2 as u64 * b as u64 + c as u64) % modulo as u64) as usize;
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
