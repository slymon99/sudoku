use std::collections::HashMap;

mod lib;

fn main() {
    let line =
        "050000000300420000006000984607100000000200070000080090400600030700000005030008410"
            .to_string();
    let mut s = sudoku_from_line(line);
    println!("\n{}", s);
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
        col_memo: HashMap::new(),
        row_memo: HashMap::new(),
        square_memo: HashMap::new(),
    };
    lib::initialize_sudoku(&mut res);
    res
}
