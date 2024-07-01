use clap::Parser;

///////////////////////////////////////////////////////////////////////////////
// Evaluation
///////////////////////////////////////////////////////////////////////////////

use std::io::{self, Read};

struct Problem {
    grid: Vec<Vec<bool>>,
    l_pos: (usize, usize),
}

fn parse_problem(maze: &str) -> Problem {
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

    Problem { grid, l_pos }
}

fn load_problem(problem_id: i64) -> Problem {
    let path = format!("input/lambdaman/lambdaman{}.txt", problem_id);
    let maze = std::fs::read_to_string(path).expect("Failed to read problem file");
    parse_problem(&maze)
}

fn run(problem: &Problem, moves: &str) -> Vec<Vec<bool>> {
    // TODO: truncate moves

    let mut visited = vec![vec![false; problem.grid[0].len()]; problem.grid.len()];
    let mut position = problem.l_pos;
    visited[position.0][position.1] = true;

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
            visited[position.0][position.1] = true;
        }
    }

    visited
}

fn get_reachable_cells(problem: &Problem) -> usize {
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

fn get_visited_cells(visited: &Vec<Vec<bool>>) -> usize {
    let mut visited_cells = 0;
    for row in visited {
        for &cell in row {
            if cell {
                visited_cells += 1;
            }
        }
    }
    visited_cells
}

///////////////////////////////////////////////////////////////////////////////
// Simulation
///////////////////////////////////////////////////////////////////////////////

fn sim(x0: i64, a: i64, c: i64, m: i64, xt: i64) -> String {
    let chars = "RRRDDDLLLUUURR".chars().collect::<Vec<_>>();
    let mut out = vec![];

    let mut x = x0;
    while x != xt {
        let k = (x % 12) as usize;
        x = (x * a + c) % m;
        out.push(chars[k]);
        out.push(chars[k + 1]);
        out.push(chars[k + 2]);
    }

    out.iter().collect::<String>()
}

fn print_visited_cells(problem: &Problem, visited: &Vec<Vec<bool>>) {
    for i in 0..problem.grid.len() {
        for j in 0..problem.grid[0].len() {
            eprint!(
                "{}",
                if (i, j) == problem.l_pos {
                    'L'
                } else if visited[i][j] {
                    'V'
                } else if problem.grid[i][j] {
                    '#'
                } else {
                    '.'
                }
            );
        }
        eprintln!();
    }
}

///////////////////////////////////////////////////////////////////////////////
// Generation
///////////////////////////////////////////////////////////////////////////////

fn get_kth(x0: i64, a: i64, c: i64, m: i64, k: i64) -> Option<i64> {
    // TODO: kを含むか後でチェック
    let mut x = x0;
    for _ in 0..k {
        x = (x * a + c) % m;
        // dbg!(&x);
    }
    let xk = x;

    // もう一回やって、k回目より前に出てきたらこのパラメタはだめ
    let mut x = x0;
    for _ in 0..k {
        if x == xk {
            return None;
        }
        x = (x * a + c) % m;
    }

    Some(x)
}

fn gen(x0: i64, a: i64, c: i64, m: i64, xt: i64, action_x: i64) -> String {
    return format!(
        r##"
B.
    "solve lambdaman20 "
    B$
        B$
            Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
            Lf Lx
            ?
                B= vx {xt}
                ""
                B.
                    ?
                        B= vx {action_x}
                        "DU"
                        ""
                    B.
                        BT 3 BD B% vx 12 "RRRDDDLLLUUURR"
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
}

fn search_special_x(
    problem: Problem,
    x0: i64,
    a: i64,
    c: i64,
    m: i64,
    xt: i64,
    target: (usize, usize),
) -> (i64, (usize, usize)) {
    let moves = "RRRDDDLLLUUURR";

    let mut visited = vec![vec![false; problem.grid[0].len()]; problem.grid.len()];
    let mut position = problem.l_pos;
    visited[position.0][position.1] = true;

    let n_reachable_cells = get_reachable_cells(&problem);
    let mut n_visited_cells = 1;

    let mut x = x0;
    let mut ret = (!0, (!0, !0));
    loop {
        let d = (position.0 as i64 - target.0 as i64).abs()
            + (position.1 as i64 - target.1 as i64).abs();
        if d == 1 {
            dbg!(
                x,
                position,
                get_visited_cells(&visited),
                get_reachable_cells(&problem),
                n_visited_cells
            );
            //return (x, position);
            ret = (x, position);
        }

        if n_visited_cells + 1 == n_reachable_cells && x < 94 * 94 {
            eprintln!("End with this!!: {x}")
        }

        let k = (x % 12) as usize;
        x = (x * a + c) % m;

        for mov in moves[k..k + 3].chars() {
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
                    visited[position.0][position.1] = true;
                    n_visited_cells += 1;
                }
            }
        }

        if x == 12703953 {
            break;
        }
    }

    dbg!(get_visited_cells(&visited), get_reachable_cells(&problem));

    ret
}

fn main() {
    let problem = load_problem(20);

    let x0 = 33;
    let a = 87;
    let c = 30;
    let m = 13014523;
    let xt = 4846; // get_kth(x0, a, c, m, 1_000_000 / 3 - 20).unwrap();
    dbg!(&xt);

    let moves = sim(x0, a, c, m, xt);
    dbg!(&moves.len());
    let visited = run(&problem, &moves);
    eprintln!(
        "{} / {}",
        get_visited_cells(&visited),
        get_reachable_cells(&problem)
    );
    // print_visited_cells(&problem, &visited);

    // visitできてない頂点を探す
    let mut target_pos = None;
    for i in 0..problem.grid.len() {
        for j in 0..problem.grid[0].len() {
            if !problem.grid[i][j] && !visited[i][j] {
                target_pos = Some((i, j));
                dbg!(target_pos);
            }
        }
    }
    let target_pos = target_pos.unwrap();

    // その隣に行くタイミングでのxを探す
    let (action_x, action_pos) = search_special_x(problem, x0, a, c, m, xt, target_pos);
    dbg!(action_x, action_pos);

    // 真上にくるっぽい
    assert_eq!(action_pos.0 + 1, target_pos.0);
    assert_eq!(action_pos.1, target_pos.1);

    let program = gen(x0, a, c, m, xt, action_x);
    println!("Program:\n{}", program);

    let compiled = icfpc2024::pp::preprocess(&program).unwrap();
    println!("Compiled:\n{}\n(len={})", compiled, compiled.len());
}
