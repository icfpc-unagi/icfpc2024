#![allow(unused)]

extern crate num_bigint;
extern crate num_traits;

use core::num;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::{self, sleep};
use std::time::Duration;

use icfpc2024::{communicate, eval};

type Board = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn move_in_direction(&self, direction: char, board: &Board) -> Option<Self> {
        let (new_x, new_y) = match direction {
            'U' => (self.x.checked_sub(1)?, self.y),
            'R' => (self.x, self.y + 1),
            'D' => (self.x + 1, self.y),
            'L' => (self.x, self.y.checked_sub(1)?),
            _ => return None,
        };
        if new_x < board.len() && new_y < board[0].len() && board[new_x][new_y] != '#' {
            Some(Position::new(new_x, new_y))
        } else {
            None
        }
    }

    fn get_neighbors(&self, board: &Board) -> Vec<(Position, char)> {
        let directions = [('U', 'U'), ('R', 'R'), ('D', 'D'), ('L', 'L')];
        directions
            .iter()
            .filter_map(|&(d, c)| self.move_in_direction(d, board).map(|pos| (pos, c)))
            .collect()
    }
}

fn read_board_from_file(filename: &str) -> io::Result<Board> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let board: Board = reader
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.trim().is_empty() {
                None
            } else {
                Some(line.chars().collect())
            }
        })
        .collect();
    Ok(board)
}

fn find_initial_position(board: &Board) -> Option<Position> {
    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'L' {
                return Some(Position::new(i, j));
            }
        }
    }
    None
}

/*
fn count_unvisited_cells(board: &Board, moves: &str) -> usize {
    let mut position = match find_initial_position(&board) {
        Some(pos) => pos,
        None => return board.iter().flatten().filter(|&&c| c == '.').count(),
    };

    let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
    visited[position.x][position.y] = true;

    for move_char in moves.chars() {
        if let Some(new_position) = position.move_in_direction(move_char, &board) {
            position = new_position;
            visited[position.x][position.y] = true;
        }
    }

    count_unvisited_dots(&board, &visited)
}
*/

fn dfs(board: &Board, pos: Position, visited: &mut Vec<Vec<bool>>, path: &mut Vec<char>) -> bool {
    visited[pos.x][pos.y] = true;

    //println!("Visiting: ({}, {})", pos.x, pos.y); // Debug information

    let neighbors = pos.get_neighbors(board);
    for (next_pos, direction) in neighbors {
        if !visited[next_pos.x][next_pos.y] {
            path.push(direction);
            dfs(board, next_pos, visited, path);

            // Backtrack
            let back_direction = match direction {
                'U' => 'D',
                'R' => 'L',
                'D' => 'U',
                'L' => 'R',
                _ => unreachable!(),
            };
            path.push(back_direction);
            // Debug information
        }
    }

    true
}

fn find_path(board: &Board) -> Option<Vec<char>> {
    let start_pos = find_initial_position(&board)?;
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    let mut path = Vec::new();

    if dfs(board, start_pos, &mut visited, &mut path) {
        Some(path)
    } else {
        None
    }
}

fn main() {
    const STACK_SIZE: usize = 512 * 1024 * 1024; // 512 MB

    let builder = thread::Builder::new().stack_size(STACK_SIZE);
    let handler = builder
        .spawn(|| {
            main2();
        })
        .unwrap();

    handler.join().unwrap();
}

fn main2() {
    for i in 1..22 {
        solve(i);
    }
}

fn solve(i: usize) {
    let filename = format!("input/lambdaman/lambdaman{}.txt", i);

    match read_board_from_file(&filename) {
        Ok(board) => {
            //eprintln!("迷路:");
            //for row in &board {
            //eprintln!("{}", row.iter().collect::<String>());
            //}
            eprintln!("task: {}", i);

            if let Some(path) = find_path(&board) {
                let moves: String = path.into_iter().collect();
                //let unvisited_count = count_unvisited_cells(&board, &moves);
                //println!("{}", moves);
                eprintln!("{}", moves.len());
                //let sendstring = format!("solve lambdaman{} {}", i, moves);
                let ss = vec![
                    //"UU".to_string(),
                    //"RR".to_string(),
                    //"DD".to_string(),
                    //"LL".to_string(),
                    "UUUU".to_string(),
                    "RRRR".to_string(),
                    "DDDD".to_string(),
                    "LLLL".to_string(),
                    "U".to_string(),
                    "R".to_string(),
                    "D".to_string(),
                    "L".to_string(),
                ];
                let sendstring = make_move2(i, &ss, &moves);

                if sendstring != "error" {
                    //eprintln!("{}", sendstring);
                    //eprintln!("{}", moves);
                    //eprintln!("{}", eval::eval(&sendstring));

                    eprintln!("length: {}", sendstring.len());
                    _ = request(&sendstring);
                    sleep(Duration::from_secs(4));
                } else {
                    eprintln!("error");
                }

                //println!("通れなかったマスの数: {}", unvisited_count);
            } else {
                //eprintln!("すべてのマスを通る経路が見つかりませんでした。");
            }
        }
        Err(e) => eprintln!("ファイルを読み込めませんでした: {}", e),
    }
}

fn make_move2(id: usize, ss: &Vec<String>, moves: &str) -> String {
    let mut num: BigInt = BigInt::ZERO + 1;

    let mut i = 0;
    loop {
        let prei = i;
        for j in 0..ss.len() {
            if ss[j].len() + i <= moves.len() && moves[i..i + ss[j].len()] == ss[j] {
                num *= ss.len() as u32;
                num += j;
                i += ss[j].len();
                break;
            }
        }
        if (prei == i) {
            return "error".to_string();
        }

        if (i == moves.len()) {
            break;
        }
    }

    let zero = "I!";
    let one = "I\"";
    // let two = "I#";
    //let three = "I$";
    let four = "I%";

    let y = "Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx";

    let mut choose_char = "".to_string();
    let div = encode_i(BigInt::ZERO + ss.len());
    for j in 0..ss.len() - 1 {
        let ej = encode_i(BigInt::ZERO + j);
        let esj = encode_str(&ss[j]);
        choose_char += &format!("? B= B% vx {div} {ej} {esj} ");
    }
    let esj_last = encode_str(&ss[ss.len() - 1]);
    choose_char += esj_last.as_str();

    // let choose_char = format!("? B= B% vx {four} {zero} {su} ? B= B% vx {four} {one} {sr} ? B= B% vx {four} {two} {sd} {sl}");
    // (take 1 (drop (v2 % 4) "URDL"))
    // f(x) = choose_char(x%4) . f(x/4)
    let f = format!("B$ {y} Lf Lx ? B> vx {one} B. B$ vf B/ vx {div} {choose_char} S");

    let program = format!("B$ {f} {}", encode_i(num));

    let first = format!("solve lambdaman{} ", id);
    let encoded_first = first.chars().map(encode).collect::<String>();
    let result = format!("B. S{} {}", encoded_first, program);

    result
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

    if s == "" {
        s = "!".to_string();
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

fn encode_str(s: &String) -> String {
    let s2 = s.chars().map(encode).collect::<String>();
    format!("S{s2}")
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

#[test]
fn test_encode_i() {
    let num = BigInt::from(1258827021845_i64);
    let encoded = encode_i(num.clone());
    let decoded = icfpc2024::eval::eval(&encoded);
    assert_eq!(format!("{}", decoded), format!("int({})", num));
}