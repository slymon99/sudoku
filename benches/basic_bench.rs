use sudoku::{initialize_sudoku, solve_sudoku, Sudoku};

fn sudoku_from_line(line: &str) -> Sudoku {
    let mut s = Sudoku {
        board: line
            .chars()
            .take(81)
            .map(|c| c.to_digit(10))
            .flatten()
            .collect(),
        row_memo: HashMap::new(),
        col_memo: HashMap::new(),
        square_memo: HashMap::new(),
    };
    initialize_sudoku(&mut s);
    s
}

fn solve_all_from_string(s: &str) -> bool {
    s.split('\n')
        .map(|line| solve_sudoku(&mut sudoku_from_line(line)))
        .all(|x| x)
}

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("sudoku");

    let line = read_file("./sudokus/easy_bench.adk").expect("Couldn't load file");
    let length = line.split("\n").collect::<Vec<_>>().len();

    group.throughput(Throughput::Elements(length as u64));
    group.sample_size(10);
    group.bench_with_input(BenchmarkId::new("solve", length), &line, |b, i| {
        b.iter(|| solve_all_from_string(i))
    });

    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
