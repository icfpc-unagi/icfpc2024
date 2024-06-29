use rustc_hash::FxHashMap;

#[allow(non_snake_case)]

pub struct OnDrop<F: Fn()> {
    f: F,
}

impl<F: Fn()> OnDrop<F> {
    #[inline]
    pub fn new(f: F) -> Self {
        OnDrop { f }
    }
}

impl<F: Fn()> Drop for OnDrop<F> {
    #[inline]
    fn drop(&mut self) {
        (self.f)()
    }
}

pub fn bench(id: String) -> OnDrop<impl Fn()> {
    eprintln!("Start({})", id);
    let t = ::std::time::SystemTime::now();
    OnDrop::new(move || {
        let d = t.elapsed().unwrap();
        let s = d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
        eprintln!("Time({}) = {:.3}", id, s);
    })
}

#[macro_export]
macro_rules! bench {
	([$name:expr]$($e: tt)*) => {
		let b = $crate::bench($name.to_owned());
		$($e)*
		drop(b);
	};
	($($e: tt)*) => {
		let b = $crate::bench(format!("{}:{}", file!(), line!()));
		$($e)*
		drop(b);
	};
}

pub static mut PROFILER: *mut Vec<(&str, &(f64, usize, usize))> = 0 as *mut Vec<_>;

#[macro_export]
macro_rules! profile {
    ($id:ident) => {
        static mut __PROF: (f64, usize, usize) = (0.0, 0, 0);
        unsafe {
            if __PROF.1 == 0 {
                if $crate::PROFILER.is_null() {
                    $crate::PROFILER = Box::into_raw(Box::new(Vec::new()));
                }
                (*$crate::PROFILER).push((stringify!($id), &__PROF));
            }
            if __PROF.2 == 0 {
                let d = ::std::time::SystemTime::now()
                    .duration_since(::std::time::SystemTime::UNIX_EPOCH)
                    .unwrap();
                __PROF.0 -= d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
            }
            __PROF.1 += 1;
            __PROF.2 += 1;
        }
        #[allow(unused)]
        let $id = $crate::OnDrop::new(move || unsafe {
            __PROF.2 -= 1;
            if __PROF.2 == 0 {
                let d = ::std::time::SystemTime::now()
                    .duration_since(::std::time::SystemTime::UNIX_EPOCH)
                    .unwrap();
                __PROF.0 += d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
            }
        });
    };
}

#[macro_export]
macro_rules! count {
    ($id:ident) => {
        static mut __PROF: (f64, usize, usize) = (0.0, 0, 0);
        unsafe {
            if __PROF.1 == 0 {
                if $crate::PROFILER.is_null() {
                    $crate::PROFILER = Box::into_raw(Box::new(Vec::new()));
                }
                (*$crate::PROFILER).push((stringify!($id), &__PROF));
            }
            __PROF.1 += 1;
        }
    };
}

pub fn write_profile() {
    let mut ps: Vec<_> = unsafe {
        if PROFILER.is_null() {
            return;
        }
        (*PROFILER).clone()
    };
    ps.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
    eprintln!("########## Profile ##########");
    for (id, &(mut t, c, depth)) in ps {
        if depth > 0 {
            let d = ::std::time::SystemTime::now()
                .duration_since(::std::time::SystemTime::UNIX_EPOCH)
                .unwrap();
            t += d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9;
        }
        eprintln!("{}:\t{:.3}\t{}", id, t, c);
    }
    eprintln!("#############################");
}

#[macro_export]
macro_rules! optuna {
	($($p:ident: $t:tt = suggest($def:expr, $($a:tt),*)),* $(,)*) => {
		#[derive(Debug, Clone)]
		struct Param {
			$($p: $t,)*
		}
		lazy_static::lazy_static! {
			static ref PARAM: Param = {
				$(let $p = std::env::var(stringify!($p)).map(|s| s.parse().expect(concat!("failed to parse ", stringify!($p)))).unwrap_or($def);)*
				Param { $( $p ),* }
			};
		}
		impl Param {
			fn optuna_str() -> String {
				let mut list = vec![];
				$(list.push($crate::optuna!(# $p, $t, $($a),*));)*
				let mut s = "def setup(trial):\n".to_owned();
				for (t, _) in &list {
					s += "\t";
					s += &t;
					s += "\n";
				}
				s += "\tenv = {";
				for (i, (_, t)) in list.iter().enumerate() {
					if i > 0 {
						s += ", ";
					}
					s += &t;
				}
				s += "}\n\treturn env";
				s
			}
		}
	};
	(# $p:ident, f64, $min:expr, $max:expr) => {
		(format!("{} = trial.suggest_float(\"{}\", {}, {})", stringify!($p), stringify!($p), $min, $max), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
	(# $p:ident, usize, $min:expr, $max:expr) => {
		(format!("{} = trial.suggest_int(\"{}\", {}, {})", stringify!($p), stringify!($p), $min, $max), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
	(# $p:ident, f64, $min:expr, $max:expr, log) => {
		(format!("{} = trial.suggest_float(\"{}\", {}, {}, log=True)", stringify!($p), stringify!($p), $min, $max), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
	(# $p:ident, f64, $min:expr, $max:expr, $step:expr) => {
		(format!("{} = trial.suggest_float(\"{}\", {}, {}, {})", stringify!($p), stringify!($p), $min, $max, $step), format!("\"{}\": str({})", stringify!($p), stringify!($p)))
	};
}

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

pub mod tsp {

    use super::*;
    use rand::prelude::*;
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

pub struct Input {
    pub ps: Vec<(i64, i64)>,
}

pub fn read_input() -> Input {
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

pub fn read_output(path: &str) -> Vec<i64> {
    let input = std::env::var("INPUT").unwrap();
    let input = std::path::Path::new(&input)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let mut out = vec![];
    for line in std::fs::read_to_string(&format!("{}/{}", path, input))
        .unwrap()
        .lines()
    {
        let line = line.trim();
        if line.len() > 0 {
            out.push(line.parse().unwrap());
        }
    }
    out
}

pub fn read_order(input: &Input, path: &str) -> Vec<usize> {
    let out = read_output(path);
    let mut order = vec![];
    let mut p = (0, 0);
    let mut v = (0, 0);
    let mut visited = vec![false; input.ps.len()];
    let mut pos = FxHashMap::default();
    for i in 0..input.ps.len() {
        pos.entry(input.ps[i]).or_insert(vec![]).push(i);
    }
    for &i in pos.get(&p).unwrap_or(&vec![]) {
        if visited[i].setmax(true) {
            order.push(i);
        }
    }
    for mv in out {
        let dx = (mv - 1) % 3 - 1;
        let dy = (mv - 1) / 3 - 1;
        v.0 += dx;
        v.1 += dy;
        p.0 += v.0;
        p.1 += v.1;
        for &i in pos.get(&p).unwrap_or(&vec![]) {
            if visited[i].setmax(true) {
                order.push(i);
            }
        }
    }
    assert_eq!(order.len(), input.ps.len());
    order
}
