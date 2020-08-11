use core::fmt;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Sudoku {
    pub board: Vec<u32>,
    pub row_memo: Vec<u32>,
    pub col_memo: Vec<u32>,
    pub square_memo: Vec<u32>,
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

pub fn initialize_sudoku(input: &mut Sudoku) {
    for idx in 0..=8 {
        input.col_memo.push(col_taken(&input.board, idx));
        input.row_memo.push(row_taken(&input.board, idx));
        input
            .square_memo
            .push(square_taken(&input.board, idx / 3 * 3, idx % 3 * 3));
    }
}

pub fn solve_sudoku(input: &mut Sudoku) -> bool {
    match find_first_empty(&input.board) {
        None => true,
        Some((row, col)) => {
            // println!(
            //     "{} \n{} {} {:?}",
            //     input,
            //     row,
            //     col,
            //     options_for(&input, row, col)
            // );
            for option in 1..=9 {
                if 1 << option - 1 & options_for(&input, row, col) == 0 {
                    let idx = (row * 9 + col) as usize;
                    input.board[idx] = option;
                    remove_option(input, row, col, option);
                    let sol = solve_sudoku(input);
                    if sol {
                        return sol;
                    }
                    input.board[idx] = 0;
                    add_option(input, row, col, option);
                }
            }
            false
        }
    }
}

fn options_for(board: &Sudoku, row: u32, col: u32) -> u32 {
    board.square_memo[row_col_to_square(row, col) as usize] | board.row_memo[row as usize] | board.col_memo[col as usize]
}

fn row_col_to_square(row: u32, col: u32) -> u32 {
    row / 3 * 3 + col / 3
}

fn remove_option(board: &mut Sudoku, row: u32, col: u32, value: u32) {
    let mask = 1 << value - 1;
    board.row_memo[row as usize] |= mask;
    board.col_memo[col as usize] |= mask;
    board.square_memo[row_col_to_square(row, col) as usize] |= mask;
}

fn add_option(board: &mut Sudoku, row: u32, col: u32, value: u32) {
    let mask = !(1 << value - 1);
    board.row_memo[row as usize] &= mask;
    board.col_memo[col as usize] &= mask;
    board.square_memo[row_col_to_square(row, col) as usize] &= mask;
}

pub fn col_taken(board: &[u32], column: u32) -> u32 {
    let mut res = 0;
    board
        .iter()
        .skip(column as usize)
        .step_by(9)
        .filter(|i| **i != 0)
        .for_each(|i| res |= 1 << *i - 1);
    res
}

pub fn row_taken(board: &[u32], row: u32) -> u32 {
    let mut res = 0;
    board
        .iter()
        .skip((row * 9) as usize)
        .take(9)
        .filter(|n| **n != 0)
        .for_each(|i| res |= 1 << *i - 1);
    res
}

pub fn square_taken(board: &[u32], row: u32, col: u32) -> u32 {
    let mut res = 0;
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
    .for_each(|i| res |= 1 << i - 1);
    res
}

fn find_first_empty(board: &[u32]) -> Option<(u32, u32)> {
    let first = board.iter().enumerate().find(|(_, item)| **item == 0)?;
    let idx = first.0 as u32;
    Some((idx / 9, idx % 9))
}
