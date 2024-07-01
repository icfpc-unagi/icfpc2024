use rand::Rng;
use std::io::{self, Read}; // Rngトレイトをインポート

struct Problem {
    grid: Vec<Vec<bool>>,
    l_pos: (usize, usize),
}

fn parse_maze(maze: &str) -> Problem {
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

fn normalize_whitespace(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char_is_space = false;

    for c in input.chars() {
        if c.is_whitespace() {
            if !prev_char_is_space {
                result.push(' ');
                prev_char_is_space = true;
            }
        } else {
            result.push(c);
            prev_char_is_space = false;
        }
    }

    result
}

fn simulate(problem: &Problem, moves: &str) -> Vec<Vec<bool>> {
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

fn print_visited_cells(problem: &Problem, visited: &Vec<Vec<bool>>) {
    for i in 0..problem.grid.len() {
        for j in 0..problem.grid[0].len() {
            print!(
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
        println!();
    }
}

/*
fn gen() -> String {
    let move1 = format!("S{}", icfpc2024::encode_str("LRDULL"));
    let move2 = format!("S{}", icfpc2024::encode_str("UDRLRR"));
    let move3 = format!("S{}", icfpc2024::encode_str("DLLLUURRDL"));
    let program = "B$ Lf B$ vf B. B. B$ vf SL B$ vf SF S> B$ Lf Ls B$ vf B$ vf B$ vf vs Ls B. B. vs vs B. vs vs";

    let program = format!(
        "\
        B$ Lf B$ vf B. B. B$ vf {move1} B$ vf {move2} {move3} \
        B$
            Lf
                Ls
                    B$ vf B$ vf B$ vf vs
        Ls
            B.
                B.
                    vs
                    BD I# vs
                B.
                    BD I\" vs
                    BD I$ vs"
    );
    dbg!(&program);

    normalize_whitespace(&program)
}
*/

fn gen(move1: &str, move2: &str, move3: &str) -> String {
    let mut s = move1.to_owned();
    for i in 0.. {
        // s = format!("{s}{move2}{s}{move3}");
        // s = format!("{}{}{}{}", s, move2, &s[0..], move3);
        // let new_s = format!("{}{}{}{}{}{}", move2, s, s, s, &s[0..], move3);
        let new_s = format!("{}{}{}{}{}{}", move2, s, s, s, s, move3);
        if new_s.len() > 1_000_000 {
            // dbg!(&i);
            break;
        }
        s = new_s;
    }
    s
}

fn gen2(move1: &str, move2: &str, move3: &str, move4: &str) -> String {
    let mut s = move1.to_owned();
    for i in 0.. {
        // s = format!("{s}{move2}{s}{move3}");
        // s = format!("{}{}{}{}", s, move2, &s[0..], move3);
        let new_s = format!("{}{}{}{}{}", move2, s, move3, &s, move4);
        if new_s.len() > 1000000 {
            dbg!(&i);
            break;
        }
        s = new_s;
    }
    s
}

fn randstr(n: usize) -> String {
    use rand::Rng;
    let chars: Vec<_> = "LRDU".chars().collect();
    (0..n)
        .map(|_| chars[rand::thread_rng().gen_range(0..4)])
        .collect()
}

fn main() {
    let mut rng = rand::thread_rng(); // スレッドに対する乱数生成器を作成

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let problem = parse_maze(&input);
    let n_reachable_cells = get_reachable_cells(&problem);

    let mut n = 0;
    let mut s = 0;
    let mut ma = 0;

    let mut best_move = vec![];
    let mut best_visited = 0;
    for k in 0..8 {
        best_move.push(randstr(6));
    }

    loop {
        let mut moves = best_move.clone();
        let k = rand::thread_rng().gen_range(0..moves.len());
        moves[k] = randstr(6);
        let k = rand::thread_rng().gen_range(0..moves.len());
        moves[k] = randstr(6);

        let mut move_str = String::new();
        for m in &moves {
            move_str += &m.repeat(64);
        }
        move_str = move_str.repeat(64);

        let visited = simulate(&problem, &move_str);
        let n_visited_cells = get_visited_cells(&visited);
        // eprintln!("{} / {}", n_visited_cells, n_reachable_cells);
        if n_visited_cells == n_reachable_cells {
            println!("{:?}", moves);
            break;
        }

        if n_visited_cells > best_visited {
            best_visited = n_visited_cells;
            best_move = moves;
        }

        // print_visited_cells(&problem, &visited);

        n += 1;
        s += n_visited_cells;
        ma = ma.max(n_visited_cells);
        if n % 100 == 0 {
            eprintln!(
                "n={:5}, avg={:.5} / max={:.5}",
                n,
                (s as f64) / (n_reachable_cells as f64 * n as f64),
                (ma as f64) / (n_reachable_cells as f64),
            );
        }
    }
}
