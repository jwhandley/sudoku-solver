use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use sudoku_solver::Sudoku;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ./sudoku-solver ./path-to-puzzles.txt");
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let puzzles: Vec<&str> = input.lines().collect();

    let now = std::time::Instant::now();
    puzzles.into_par_iter().progress().for_each(|puzzle| {
        let sudoku = Sudoku::parse(puzzle);
        let solution = sudoku.solve();

        assert!(solution.unwrap().is_valid());
    });

    let total = now.elapsed();
    println!("Total solve time: {:?}", total);
}
