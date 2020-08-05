use core::fmt;
use std::collections::HashSet;
use std::fmt::Formatter;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ALL_OPTIONS: HashSet<&'static u32> =
        [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect();
}

fn main() {
    // sudokus_from_files("sudokus/0_Easy_000.adk");
    let line =
        "600079020940103000000400000000002054205806100070000000000000000000600538400000063 #Easy"
            .to_string();
    let s = sudoku_from_line(line);
    println!("{}", s);
    println!("{:?}", square_taken(&s.board, 3, 4));
    println!("{:?}", find_first_empty(&s.board));
    solve_sudoku(s);
}

fn solve_sudoku(input: Sudoku) -> Option<Sudoku> {
    match find_first_empty(&input.board) {
        None => Some(input),
        Some((row, col)) => {
            let options = &ALL_OPTIONS
                .difference(&col_taken(&input.board, col))
                .copied()
                .collect::<HashSet<&u32>>()
                .difference(&row_taken(&input.board, row))
                .copied()
                .collect::<HashSet<&u32>>()
                .difference(&square_taken(&input.board, row, col))
                .copied()
                .collect::<Vec<&u32>>();
            println!("{:?}", options);
            None
        }
    }
}

fn sudoku_from_line(line: String) -> Sudoku {
    Sudoku {
        board: line
            .chars()
            .take(81)
            .map(|c| c.to_digit(10))
            .flatten()
            .collect(),
    }
}

fn col_taken(board: &[u32], column: u32) -> HashSet<&u32> {
    board
        .iter()
        .skip(column as usize)
        .step_by(9)
        .filter(|n| **n != 0)
        .collect()
}

fn row_taken(board: &[u32], row: u32) -> HashSet<&u32> {
    board
        .iter()
        .skip((row * 9) as usize)
        .take(9)
        .filter(|n| **n != 0)
        .collect()
}

fn square_taken(board: &[u32], row: u32, col: u32) -> HashSet<&u32> {
    let start = (row / 3 * 27 + col / 3 * 3) as usize;
    [
        &board[start..start + 3],
        &board[start + 9..start + 12],
        &board[start + 18..start + 21],
    ]
        .to_vec()
        .iter()
        .map(|slice| slice.iter())
        .flatten()
        .filter(|n| **n != 0)
        .collect()
}

fn find_first_empty(board: &[u32]) -> Option<(u32, u32)> {
    let first = board.iter().enumerate().find(|(_, item)| **item == 0)?;
    let idx = first.0 as u32;
    Some((idx / 9, idx % 9))
}

struct Sudoku {
    board: Vec<u32>,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            &self
                .board
                .chunks(9)
                .map(|row| row
                    .to_vec()
                    .chunks(3)
                    .map(|block| block
                        .to_vec()
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<_>>()
                        .join(" "))
                    .collect::<Vec<_>>()
                    .join(" | "))
                .collect::<Vec<_>>()
                .chunks(3)
                .map(|group_of_rows| group_of_rows.to_vec().join("\n"))
                .collect::<Vec<_>>()
                .join("\n---------------------\n")
        )
    }
}
