use core::array;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::time::Duration;

use indicatif::ProgressBar;

type Peers = [[[(usize, usize); 20]; 9]; 9];

static PEERS: LazyLock<Peers> =
    LazyLock::new(|| array::from_fn(|row| array::from_fn(|col| get_peers(row, col))));

fn get_peers(row: usize, col: usize) -> [(usize, usize); 20] {
    let mut peers = [(0, 0); 20];
    let mut i = 0;

    // First 8
    for r in 0..9 {
        if r != row {
            peers[i] = (r, col);
            i += 1;
        }
    }

    // Second 8
    for c in 0..9 {
        if c != col {
            peers[i] = (row, c);
            i += 1;
        }
    }

    // Remaining four
    let box_r = (row / 3) * 3;
    let box_c = (col / 3) * 3;

    for dr in 0..3 {
        for dc in 0..3 {
            let r = box_r + dr;
            let c = box_c + dc;

            if r != row && c != col {
                peers[i] = (r, c);
                i += 1;
            }
        }
    }

    assert!(i == 20);
    peers
}

#[derive(Clone, Copy)]
enum Cell {
    Solved(u32),
    Uncertain(u16),
}

fn to_bit(v: u32) -> u16 {
    1 << (v - 1)
}
fn count_bits(x: u16) -> u32 {
    x.count_ones()
}

fn possible_values(mask: u16) -> impl Iterator<Item = u32> {
    (1..=9).filter(move |&v| mask & to_bit(v) != 0)
}

impl Cell {
    fn remove(&mut self, value: u32) -> bool {
        match self {
            Cell::Solved(v) => *v != value,
            Cell::Uncertain(bits) => {
                *bits &= !to_bit(value);
                *bits != 0
            }
        }
    }

    fn one_left(&self) -> Option<u32> {
        match self {
            Cell::Solved(_) => None,
            Cell::Uncertain(bits) if bits.count_ones() == 1 => Some(bits.trailing_zeros() + 1),
            _ => None,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::Uncertain(0b111111111)
    }
}

#[derive(Clone, Copy)]
struct Sudoku {
    cells: [Cell; 81],
}

impl Default for Sudoku {
    fn default() -> Self {
        let cells = [Cell::default(); 81];
        Self { cells }
    }
}

impl Sudoku {
    fn parse(input: &str) -> Self {
        let mut result = Self::default();

        let input: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

        for r in 0..9usize {
            for c in 0..9 {
                let idx = r * 9 + c;
                if let Some(value) = input[idx].to_digit(10) {
                    result.set(r, c, value);
                }
            }
        }

        result
    }

    fn is_valid(&self) -> bool {
        let mut seen = HashSet::new();

        for r in 0..9 {
            seen.clear();

            for c in 0..9 {
                if let Cell::Solved(v) = self.cells[r * 9 + c] {
                    if !seen.insert(v) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        for c in 0..9 {
            seen.clear();

            for r in 0..9 {
                if let Cell::Solved(v) = self.cells[r * 9 + c] {
                    if !seen.insert(v) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        for box_r in 0..3 {
            let box_r = box_r * 3;
            for box_c in 0..3 {
                let box_c = box_c * 3;

                seen.clear();
                for dr in 0..3 {
                    for dc in 0..3 {
                        let idx = (box_r + dr) * 9 + (box_c + dc);
                        if let Cell::Solved(v) = self.cells[idx] {
                            if !seen.insert(v) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn set(&mut self, row: usize, col: usize, value: u32) -> bool {
        let idx = row * 9 + col;
        self.cells[idx] = Cell::Solved(value);

        for (nr, nc) in PEERS[row][col].iter() {
            let nbr = nr * 9 + nc;
            if !self.cells[nbr].remove(value) {
                return false;
            }

            if let Some(v) = self.cells[nbr].one_left() {
                if !self.set(*nr, *nc, v) {
                    return false;
                }
            }
        }

        true
    }

    fn solve(&self) -> Option<Self> {
        let mut count = u32::MAX;
        let mut best_idx = None;

        for r in 0..9usize {
            for c in 0..9 {
                let idx = r * 9 + c;
                match &self.cells[idx] {
                    Cell::Solved(_) => {
                        continue;
                    }
                    Cell::Uncertain(possible) => {
                        if count_bits(*possible) < count {
                            count = count_bits(*possible);
                            best_idx = Some((r, c));
                        }
                    }
                }
            }
        }

        match best_idx {
            Some((r, c)) => {
                let idx = r * 9 + c;
                if let Cell::Uncertain(possible) = &self.cells[idx] {
                    possible_values(*possible).find_map(|p| {
                        let mut next = *self;

                        if !next.set(r, c, p) {
                            return None;
                        }

                        next.solve()
                    })
                } else {
                    None
                }
            }
            _ => Some(*self),
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..9usize {
            for c in 0..9 {
                let idx = r * 9 + c;
                match self.cells[idx] {
                    Cell::Solved(v) => write!(f, "{v}")?,
                    Cell::Uncertain(_) => write!(f, ".")?,
                }
            }

            writeln!(f)?
        }

        Ok(())
    }
}

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
