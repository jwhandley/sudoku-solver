use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::time::Duration;
use sudoku_solver::Sudoku;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ./sudoku-solver ./path-to-puzzles.txt");
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let puzzles: Vec<&str> = input.lines().collect();

    let times: Vec<_> = puzzles
        .into_par_iter()
        .progress()
        .map(|puzzle| {
            let start = std::time::Instant::now();

            let sudoku = Sudoku::parse(puzzle);
            let solution = sudoku.solve().unwrap();
            assert!(solution.is_valid());

            start.elapsed()
        })
        .collect();

    let total = times.iter().sum::<Duration>();
    println!("Average solve time: {:?}", total / times.len() as u32);
}
