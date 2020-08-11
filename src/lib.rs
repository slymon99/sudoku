use core::fmt;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Sudoku {
    pub board: Vec<u32>,
    pub row_memo: HashMap<u32, HashSet<u32>>,
    pub col_memo: HashMap<u32, HashSet<u32>>,
    pub square_memo: HashMap<(u32, u32), HashSet<u32>>,
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
        input
            .col_memo
            .insert(idx, flip(&col_taken(&input.board, idx)));
        input
            .row_memo
            .insert(idx, flip(&row_taken(&input.board, idx)));
        let (row, col) = (idx / 3, idx % 3);
        input.square_memo.insert(
            (row, col),
            flip(&square_taken(&input.board, row * 3, col * 3)),
        );
    }
}

fn flip(input: &HashSet<u32>) -> HashSet<u32> {
    (1..=9)
        .collect::<HashSet<u32>>()
        .difference(input)
        .copied()
        .collect::<HashSet<_>>()
}

pub fn solve_sudoku(input: &mut Sudoku) -> bool {
    match find_first_empty(&input.board) {
        None => true,
        Some((row, col)) => {
            let options = options_for(&input, row, col);
            // println!(
            //     "{} \n{} {} {:?} {:?}",
            //     input,
            //     row,
            //     col,
            //     options,
            //     input.row_memo
            // );
            for option in &options {
                let idx = (row * 9 + col) as usize;
                input.board[idx] = *option;
                remove_option(input, row, col, *option);
                let sol = solve_sudoku(input);
                if sol {
                    return sol;
                }
                input.board[idx] = 0;
                add_option(input, row, col, *option);
            }
            false
        }
    }
}

fn options_for(board: &Sudoku, row: u32, col: u32) -> HashSet<u32> {
    board
        .row_memo
        .get(&row)
        .unwrap()
        .intersection(&board.col_memo.get(&col).unwrap())
        .copied()
        .collect::<HashSet<u32>>()
        .intersection(&board.square_memo.get(&(row / 3, col / 3)).unwrap())
        .copied()
        .collect()
}

fn remove_option(board: &mut Sudoku, row: u32, col: u32, value: u32) {
    board
        .square_memo
        .get_mut(&(row / 3, col / 3))
        .unwrap()
        .remove(&value);
    board.row_memo.get_mut(&row).unwrap().remove(&value);
    board.col_memo.get_mut(&col).unwrap().remove(&value);
}

fn add_option(board: &mut Sudoku, row: u32, col: u32, value: u32) {
    board
        .square_memo
        .get_mut(&(row / 3, col / 3))
        .unwrap()
        .insert(value);
    board.row_memo.get_mut(&row).unwrap().insert(value);
    board.col_memo.get_mut(&col).unwrap().insert(value);
}

fn col_taken(board: &[u32], column: u32) -> HashSet<u32> {
    board
        .iter()
        .skip(column as usize)
        .step_by(9)
        .filter(|n| **n != 0)
        .copied()
        .collect()
}

fn row_taken(board: &[u32], row: u32) -> HashSet<u32> {
    board
        .iter()
        .skip((row * 9) as usize)
        .take(9)
        .filter(|n| **n != 0)
        .copied()
        .collect()
}

fn square_taken(board: &[u32], row: u32, col: u32) -> HashSet<u32> {
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
    .copied()
    .collect()
}

fn find_first_empty(board: &[u32]) -> Option<(u32, u32)> {
    let first = board.iter().enumerate().find(|(_, item)| **item == 0)?;
    let idx = first.0 as u32;
    Some((idx / 9, idx % 9))
}
