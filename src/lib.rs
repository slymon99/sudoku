use core::fmt;
use std::fmt::Formatter;
use std::collections::HashSet;

pub struct Sudoku {
    pub board: Vec<u32>,
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

pub fn solve_sudoku(input: &mut Sudoku) -> bool {
    match find_first_empty(&input.board) {
        None => true,
        Some((row, col)) => {
            // println!("{} \n {} {} {:?}", input, row, col, options_for(&input.board, row, col));
            for option in  options_for(&input.board, row, col){
                let idx = (row * 9 + col) as usize;
                input.board[idx] = option;
                let sol = solve_sudoku(input);
                if sol {
                    return sol;
                }
                input.board[idx] = 0;
            }
            return false;
        }
    }
}

fn options_for(board: &[u32], row: u32, col: u32) -> Vec<u32> {
    (1..=9).collect::<HashSet<u32>>()
        .difference(&col_taken(board, col))
        .copied()
        .collect::<HashSet<u32>>()
        .difference(&row_taken(board, row))
        .copied()
        .collect::<HashSet<u32>>()
        .difference(&square_taken(board, row, col))
        .copied()
        .collect::<Vec<_>>()
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

