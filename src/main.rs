mod lib;

fn main() {
    let line =
        "000000000549000007200800600054270000006003009020950000400600003080020750005000000 #Easy"
            .to_string();
    let mut s = sudoku_from_line(line);
    println!("\n{}", s);
    println!("Solving!");
    lib::solve_sudoku(&mut s);
    println!("\n{}", s)
    // println!("{:?}", HashSet::<u32>::from_iter((1..=2).into_iter()));
}

fn sudoku_from_line(line: String) -> lib::Sudoku {
    lib::Sudoku {
        board: line
            .chars()
            .take(81)
            .map(|c| c.to_digit(10))
            .flatten()
            .collect(),
    }
}
