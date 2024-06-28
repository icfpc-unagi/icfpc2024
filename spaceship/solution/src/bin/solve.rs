#![allow(non_snake_case)]

fn main() {
    get_time();
    let input = read_input();
    let mut visited = vec![false; input.ps.len()];
    let mut p = (0, 0);
    let mut v = (0, 0);
    let mut out = vec![];
    for _ in 0..input.ps.len() {
        let mut to = !0;
        let mut min = i64::MAX;
        for i in 0..input.ps.len() {
            if !visited[i] && min.setmin((p.0 - input.ps[i].0).abs() + (p.1 - input.ps[i].1).abs())
            {
                to = i;
            }
        }
        visited[to] = true;
        if min == 0 {
            continue;
        }
        let dx = input.ps[to].0 - p.0;
        let dy = input.ps[to].1 - p.1;
        let mut T = 1;
        loop {
            if v.0 * T - T * (T + 1) / 2 <= dx && dx <= v.0 * T + T * (T + 1) / 2 {
                if v.1 * T - T * (T + 1) / 2 <= dy && dy <= v.1 * T + T * (T + 1) / 2 {
                    break;
                }
            }
            T += 1;
        }
        for t in 0..T {
            let dvx = if p.0 + v.0 * (T - t) < input.ps[to].0 {
                1
            } else if p.0 + v.0 * (T - t) > input.ps[to].0 {
                -1
            } else {
                0
            };
            let dvy = if p.1 + v.1 * (T - t) < input.ps[to].1 {
                1
            } else if p.1 + v.1 * (T - t) > input.ps[to].1 {
                -1
            } else {
                0
            };
            v.0 += dvx;
            v.1 += dvy;
            p.0 += v.0;
            p.1 += v.1;
            out.push((dvy + 1) * 3 + dvx + 1 + 1);
        }
    }
    for mv in out {
        println!("{}", mv);
    }
    eprintln!("Time = {:.3}", get_time());
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
