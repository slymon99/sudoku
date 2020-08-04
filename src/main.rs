use std::io;
use std::fmt::{Error, Formatter};
use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use core::fmt;
use std::collections::HashSet;

fn main() {
    // sudokus_from_files("sudokus/0_Easy_000.adk");
    let line = "600079020940103000000400000000002054205806100070000000000000000000600538400000063 #Easy".to_string();
    let s = sudoku_from_line(line);
    println!("{}", s);
    println!("{:?}", square(s, 3, 4));
}

fn sudoku_from_line(line: String) -> Sudoku {
    return Sudoku{board: line.chars()
        .take(81)
        .map(|c| c.to_digit(10))
        .flatten()
        .collect()};
}

fn column(input: Sudoku, column: u32) -> HashSet<u32> {
    return input.board
        .into_iter()
        .skip(column as usize)
        .step_by(9)
        .filter(|n| *n != 0)
        .collect();
}

fn row(input: Sudoku, row: u32) -> HashSet<u32> {
    return input.board
        .into_iter()
        .skip((row * 9) as usize)
        .take(9)
        .filter(|n| *n != 0)
        .collect();
}

fn square(input: Sudoku, row: u32, col: u32) -> HashSet<u32> {
    let start = (row / 3 * 27 + col / 3 * 3) as usize;
    return [&input.board[start..start+3],
        &input.board[start+9..start+12],
        &input.board[start+18..start+21]]
        .concat()
        .to_vec()
        .into_iter()
        .filter(|n| *n != 0)
        .collect();
}

struct Sudoku{board: Vec<u32>}

impl fmt::Display for Sudoku{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.board
            .chunks(9)
            .map(|row| row
                .to_vec()
                .chunks(3)
                .map(|block| block
                    .to_vec()
                    .into_iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join(" | "))
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group_of_rows| group_of_rows
                .to_vec()
                .join("\n"))
            .collect::<Vec<_>>()
            .join("\n---------------------\n"))
    }
}

