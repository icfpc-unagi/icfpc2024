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

fn sim1(x0: i64, a: i64, c: i64, m: i64, xt: i64) -> String {
    let chars = "URDL".chars().collect::<Vec<_>>();
    let mut out = vec![];

    let mut x = x0;
    while x != xt {
        x = (x * a + c) % m;
        out.push(chars[(x % 4) as usize]);
    }

    out.iter().collect::<String>()
}

fn sim2(x0: i64, a: i64, c: i64, m: i64, xt: i64) -> String {
    let chars = "URDL".chars().collect::<Vec<_>>();
    let mut out = vec![];

    let mut x = x0;
    while x != xt {
        x = (x * a + c) % m;
        out.push(chars[(x % 4) as usize]);
        out.push(chars[(x % 4) as usize]);
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

fn gen1(problem_id: i64, x0: i64, a: i64, c: i64, m: i64, xt: i64) -> String {
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
                    BT 1 BD B% vx 4 "URDL"
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

fn gen2(problem_id: i64, x0: i64, a: i64, c: i64, m: i64, xt: i64) -> String {
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
                    BT 2 BD B% B* vx 2 8 "UURRDDLLU"
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

/// A simple program to send file contents as requests
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to read files from
    #[arg(long)]
    problem: i64,
}

fn main() {
    {
        let x0 = 54;
        let a = 81;
        let c = 91;
        let m = 1000033;
        let xt = get_kth(x0, a, c, m, 500_000).unwrap();
        dbg!(&xt);
    }

    let args = Args::parse();
    dbg!(&args);

    let x0 = 10;
    let a = 23;
    let c = 9;
    let m = 1000003;
    let xt = get_kth(x0, a, c, m, 500_000).unwrap();

    let problem = load_problem(args.problem);
    let moves = sim2(x0, a, c, m, xt);
    let visited = run(&problem, &moves);
    print_visited_cells(&problem, &visited);
    eprintln!(
        "{} / {}",
        get_visited_cells(&visited),
        get_reachable_cells(&problem)
    );
    return;

    // let xt = get_kth(x0, a, c, m, 1_000_000).unwrap();
    // let program = gen1(5, x0, a, c, m, xt);

    // dryrun
    {
        let xt = get_kth(x0, a, c, m, 10).unwrap();
        let program = gen1(args.problem, x0, a, c, m, xt);
        let compiled = icfpc2024::pp::preprocess(&program).unwrap();
        eprintln!("Dryrun:\n{}", icfpc2024::eval::eval(&compiled).unwrap());
    }

    let xt = get_kth(x0, a, c, m, 500_000).unwrap();
    let program = gen1(args.problem, x0, a, c, m, xt);
    eprintln!("{}", program);

    let compiled = icfpc2024::pp::preprocess(&program).unwrap();
    println!("{}", compiled);
}
