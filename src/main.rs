use std::collections::HashMap;

mod lib;

fn main() {
    let line = "050000000300420000006000984607100000000200070000080090400600030700000005030008410"
        .to_string();
    let mut s = sudoku_from_line(line);
    // println!("{}", lib::square_taken(&s.board, 0, 0));
    // println!("{:?}", s.row_memo);
    println!("Solving!");
    lib::solve_sudoku(&mut s);
    println!("\n{}", s);
}

fn sudoku_from_line(line: String) -> lib::Sudoku {
    let mut res = lib::Sudoku {
        board: line
            .chars()
            .take(81)
            .map(|c| c.to_digit(10))
            .flatten()
            .collect(),
        col_memo: Vec::new(),
        row_memo: Vec::new(),
        square_memo: Vec::new(),
    };
    lib::initialize_sudoku(&mut res);
    res
}
