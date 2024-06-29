use anyhow::Context;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Board = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy)]
pub struct Position {
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
}

pub fn find_initial_position(board: &Board) -> Option<Position> {
    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'L' {
                return Some(Position::new(i, j));
            }
        }
    }
    None
}

pub fn simulate(board: &Board, moves: &str) -> Board {
    let mut position = match find_initial_position(&board) {
        Some(pos) => pos,
        None => return board.clone(),
    };

    let mut board = board.clone();
    board[position.x][position.y] = '*';

    for move_char in moves.chars() {
        if let Some(new_position) = position.move_in_direction(move_char, &board) {
            position = new_position;
            board[position.x][position.y] = '*';
        }
    }
    board[position.x][position.y] = 'L';
    board
}

fn read_board_from_file(filename: &str) -> anyhow::Result<Board> {
    let path = Path::new(filename);
    let file = File::open(&path).context(format!("Failed to open file: {}", filename))?;
    let reader = io::BufReader::new(file);
    let board: Board = reader
        .lines()
        .filter_map(|line| match line {
            Ok(x) if !x.trim().is_empty() => Some(x.chars().collect()),
            _ => None,
        })
        .collect();
    Ok(board)
}

pub fn simulate_with_problem_id(problem_id: i64, moves: &str) -> Board {
    let filename: String = format!("input/lambdaman/lambdaman{}.txt", problem_id);

    let board = read_board_from_file(&filename).unwrap();
    simulate(&board, moves)
}
