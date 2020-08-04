use std::io;
use std::fmt::{Error, Formatter};
use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use core::fmt;

fn main() {
    // sudokus_from_files("sudokus/0_Easy_000.adk");
    let line = "600079020940103000000400000000002054205806100070000000000000000000600538400000063 #Easy".to_string();
    let v = sudoku_from_line(line);
    println!("{:?}", v.board.len());
}

fn sudoku_from_line(line: String) -> Sudoku {
    return Sudoku{board: line.chars()
        .take(81)
        .map(|c| c.to_digit(10))
        .flatten()
        .collect()};
}




struct Sudoku{board: Vec<u32>}

impl fmt::Display for Sudoku{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board[0])
    }
}

