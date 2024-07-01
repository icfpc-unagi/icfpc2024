use clap::Parser;
use itertools::Itertools;
use rand::prelude::*;
use rand::seq::SliceRandom;

use rand::thread_rng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use std::io::{self, Read};

use std::sync::Mutex;
///////////////////////////////////////////////////////////////////////////////
// Problem
///////////////////////////////////////////////////////////////////////////////

struct Problem {
    id: i64,
    grid: Vec<Vec<bool>>,
    l_pos: (usize, usize),
}

fn parse_problem(maze: &str, id: i64) -> Problem {
    let lines: Vec<&str> = maze.trim().lines().collect();
    let mut grid: Vec<Vec<bool>> = vec![vec![true; lines[0].len() + 2]; lines.len() + 2];
    let mut l_pos: (usize, usize) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            grid[i + 1][j + 1] = match char {
                '#' => true,
                '.' => false,
                'L' => {
                    l_pos = (i + 1, j + 1);
                    false
                }
                _ => continue,
            };
        }
    }

    Problem { id, grid, l_pos }
}

fn load_problem(problem_id: i64) -> Problem {
    let path = format!("input/lambdaman/lambdaman{}.txt", problem_id);
    let maze = std::fs::read_to_string(path).expect("Failed to read problem file");
    parse_problem(&maze, problem_id)
}

fn get_reachable_cells(problem: &Problem) -> i64 {
    let mut reachable_cells = 0;
    for row in &problem.grid {
        for &cell in row {
            if !cell {
                reachable_cells += 1;
            }
        }
    }
    reachable_cells
}

fn run(problem: &Problem, moves: &str) -> i64 {
    let mut visited = vec![vec![false; problem.grid[0].len()]; problem.grid.len()];
    let mut position = problem.l_pos;
    visited[position.0][position.1] = true;
    let mut n_visited_cells = 1;

    for mov in moves.chars() {
        let new_position = match mov {
            'L' => (position.0, position.1.saturating_sub(1)),
            'R' => (position.0, position.1 + 1),
            'U' => (position.0.saturating_sub(1), position.1),
            'D' => (position.0 + 1, position.1),
            _ => position,
        };

        if !problem.grid[new_position.0][new_position.1] {
            position = new_position;
            if !visited[position.0][position.1] {
                n_visited_cells += 1;
                visited[position.0][position.1] = true;
            }
        }
    }

    n_visited_cells
}

///////////////////////////////////////////////////////////////////////////////
// LCG
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
struct LCGConfig {
    x0: i64,
    a: i64,
    m: i64,
    xt: i64,
}

fn generate_moves(lcg: &LCGConfig, step: i64) -> Vec<char> {
    let chars = "RDLU".chars().collect::<Vec<_>>();

    let mut moves = vec![];
    let mut x = lcg.x0;
    loop {
        if x == lcg.xt {
            break;
        }

        let d = x % 4;
        for _ in 0..step {
            moves.push(chars[d as usize]);
        }

        x = x * lcg.a % lcg.m;
    }

    moves.reverse();
    moves
}

///////////////////////////////////////////////////////////////////////////////
// Program generation
///////////////////////////////////////////////////////////////////////////////

fn gen_step1(problem_id: i64, lcg: &LCGConfig) -> String {
    let x0 = lcg.x0;
    let a = lcg.a;
    let m = lcg.m;
    let xt = lcg.xt;

    format!(
        r##"
            B$
                B$
                    Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
                    Lf Lx
                    ?
                        B= vx {xt}
                        "solve lambdaman{problem_id} "
                        B.
                            B$
                                vf
                                B%
                                    B*
                                        vx
                                        {a}
                                    {m}
                            BT 1 BD B% vx 4 "RDLU"
                {x0}
    "##
    )
}

fn gen_step2(problem_id: i64, lcg: &LCGConfig) -> String {
    let a = lcg.a;
    let dm = lcg.m * 2;
    let dx0 = lcg.x0 * 2;
    let dxt = lcg.xt * 2;

    format!(
        r##"
            B$
                B$
                    Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
                    Lf Lx
                    ?
                        B= vx {dxt}
                        "solve lambdaman{problem_id} "
                        B.
                            B$ vf
                                B%
                                    B*
                                        vx
                                        {a}
                                    {dm}
                            BT 2 BD B% vx 8 "RRDDLLUU"
                {dx0}
    "##
    )
}

fn gen(problem_id: i64, lcg: &LCGConfig, step: i64) -> String {
    if step == 1 {
        gen_step1(problem_id, lcg)
    } else {
        gen_step2(problem_id, lcg)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Simulation
///////////////////////////////////////////////////////////////////////////////

fn simulate(problem: &Problem, lcg: &LCGConfig, step: i64) -> i64 {
    let inv_a = inverse(lcg.a, lcg.m);
    let chars = "RDLU".chars().collect::<Vec<_>>();

    let mut pos = problem.l_pos.clone();
    let mut visited = vec![vec![false; problem.grid[0].len()]; problem.grid.len()];
    let mut n_visited_cells = 1;
    visited[pos.0][pos.1] = true;

    let mut x = lcg.xt;
    loop {
        x = (x * inv_a) % lcg.m;

        let d = x % 4;
        for _ in 0..step {
            let c = chars[d as usize];

            let nxt_pos = match c {
                'L' => (pos.0, pos.1.saturating_sub(1)),
                'R' => (pos.0, pos.1 + 1),
                'U' => (pos.0.saturating_sub(1), pos.1),
                'D' => (pos.0 + 1, pos.1),
                _ => unreachable!(),
            };

            if !problem.grid[nxt_pos.0][nxt_pos.1] {
                pos = nxt_pos;
                if !visited[pos.0][pos.1] {
                    n_visited_cells += 1;
                    visited[pos.0][pos.1] = true;
                }
            }
        }

        if x == lcg.x0 {
            break;
        }
    }

    n_visited_cells
}

///////////////////////////////////////////////////////////////////////////////
// Number theory
///////////////////////////////////////////////////////////////////////////////

fn is_prime(x: i64) -> bool {
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn next_prime(x: i64) -> i64 {
    let mut x = x + 1;
    while !is_prime(x) {
        x += 1;
    }
    x
}

fn generate_modulo_candidates(min: i64, n: usize) -> Vec<i64> {
    let mut candidates = vec![];
    let mut x = min;
    while candidates.len() < n {
        if is_prime(x) {
            candidates.push(x);
        }
        x += 1;
    }

    candidates
}

/*
typedef long long ll;

inline ll mod(ll a, ll m) { return (a % m + m) % m; }

ll inverse(ll a, ll m) {
  if ((a = mod(a, m)) == 1) return 1;
  return mod((1 - m * inverse(m % a, a)) / a, m);
}
*/
fn inverse(a: i64, m: i64) -> i64 {
    let a = a % m;
    if a == 1 {
        return 1;
    }
    ((1 - m * inverse(m % a, a)) / a % m + m) % m
}

fn find_x0(xt: i64, a: i64, m: i64, step: i64) -> Option<i64> {
    let limit = 1_000_000 - 30;
    let inv_a = inverse(a, m);

    let mut x0_i = None;
    let mut len = 0;

    // 1回目のシミュレートでx0とそのiterを記録する
    let mut x = xt;
    for i in 0.. {
        x = (x * inv_a) % m;

        if x * step < 94 {
            x0_i = Some((x, i));
        }

        len += step;
        if len > limit {
            break;
        }
    }
    if x0_i.is_none() {
        return None;
    }
    let (x0, last_i) = x0_i.unwrap();

    // 2回目のシミュレートはi回目になるかどうかの確認
    let mut x = xt;
    for i in 0.. {
        x = (x * inv_a) % m;

        if x == x0 {
            if i == last_i {
                return Some(x0);
            } else {
                return None;
            }
        }
    }

    panic!();
}

///////////////////////////////////////////////////////////////////////////////
// Main
///////////////////////////////////////////////////////////////////////////////

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    problem: i64,

    #[arg(long)]
    step: i64,

    #[arg(long, default_value_t = 1000)]
    batch_size: usize,

    #[arg(long, default_value_t = 1000003)]
    min_m: i64,
}

fn finish(problem_id: i64, lcg: &LCGConfig, step: i64) {
    eprintln!("================================================================================");
    eprintln!("Problem: {problem_id}");
    eprintln!("LCG: {lcg:?}");
    let program = gen(problem_id, lcg, step);
    eprintln!("Program:\n{}", &program);
    let compiled = icfpc2024::pp::preprocess(&program).unwrap();
    println!("Compiled:\n{}\n(len={})", &compiled, compiled.len());
}

fn doit(problem: &Problem, step: i64, m: i64, global_best_score: &Mutex<i64>) -> i64 {
    let n_reachable_cells = get_reachable_cells(&problem);

    // eprintln!("Modulo: {m}");
    /*
    let mut lcg_configs = vec![];
    for x0 in 1..=(93 / step) {
        for a in 2..=93 {
            lcg_configs.push(LCGConfig {
                x0,
                a,
                c: 0,
                m,
                xt: None,
            })
        }
    }
    lcg_configs.shuffle(&mut rand::thread_rng());
    */

    let mut local_best_score = 9999999;
    for a in 2..=93 {
        for xt in 1..=(93 / step) {
            // x0を発見する。ついでに、原始根でなければ枝狩りされるはず。
            let x0 = find_x0(xt, a, m, step);
            if x0.is_none() {
                continue;
            }
            let x0 = x0.unwrap();

            let lcg = LCGConfig { x0, a, m, xt };

            let n_visited_cells = simulate(problem, &lcg, step);

            // TODO: remove me!
            /*
            let moves = generate_moves(&lcg, step);
            let n_visited_cells2 = run(&problem, &moves.iter().collect::<String>());
            assert_eq!(n_visited_cells, n_visited_cells2);
            */

            let score = n_reachable_cells - n_visited_cells;
            if score < local_best_score {
                let mut gbs = global_best_score.lock().unwrap();
                if score < *gbs {
                    *gbs = score;
                    eprintln!("M={:8} | remain={:3}", lcg.m, score);

                    if score == 0 {
                        finish(problem.id, &lcg, step);
                    }
                }
                local_best_score = *gbs;
            }

            // dbg!(n_visited_cells, n_reachable_cells);
            if local_best_score == 0 {
                return 0;
            }
        }
    }

    local_best_score
}

fn main() {
    let args = Args::parse();
    let problem = load_problem(args.problem);

    let mut min_m: i64 = args.min_m;
    let global_best_score = Mutex::new(9999999);
    loop {
        let m_cands = generate_modulo_candidates(min_m, args.batch_size);
        eprintln!("[Batch: {}...{}]", m_cands[0], m_cands[m_cands.len() - 1]);
        min_m = m_cands.iter().max().unwrap() + 1; // for next batch

        m_cands.into_par_iter().for_each(|m| {
            if *global_best_score.lock().unwrap() == 0 {
                return;
            }

            let n_remaining_cells = doit(&problem, args.step, m, &global_best_score);
            {
                let mut b = global_best_score.lock().unwrap();
                if n_remaining_cells < *b {
                    *b = n_remaining_cells;
                }
            }
        });

        let best_remaining_cells = *global_best_score.lock().unwrap();
        if best_remaining_cells == 0 {
            break;
        }
    }
}