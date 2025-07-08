use indicatif::ProgressBar;
use std::time::Duration;
use sudoku_solver::Sudoku;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ./sudoku-solver ./path-to-puzzles.txt");
    }

    let mut times = Vec::new();
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let puzzles: Vec<_> = input.lines().collect();
    let pb: ProgressBar = ProgressBar::new(puzzles.len() as u64);
    for puzzle in puzzles.iter() {
        let start = std::time::Instant::now();

        let sudoku = Sudoku::parse(puzzle);
        let solution = sudoku.solve().unwrap();
        assert!(solution.is_valid());

        times.push(start.elapsed());
        pb.inc(1);
    }

    let total = times.iter().sum::<Duration>();
    pb.finish();
    println!("Average solve time: {:?}", total / times.len() as u32);
}
