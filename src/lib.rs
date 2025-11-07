use std::collections::HashSet;
mod peers;

#[derive(Clone, Copy, Debug)]
struct Cell {
    possible: u16,
}

impl Cell {
    fn remove(&mut self, value: u32) -> bool {
        self.possible &= !(1 << (value - 1));
        self.possible != 0
    }

    fn is_solved(&self) -> Option<u32> {
        if self.count() == 1 {
            Some(self.possible.trailing_zeros() + 1)
        } else {
            None
        }
    }

    fn count(&self) -> usize {
        self.possible.count_ones() as usize
    }

    fn possible_values(&self) -> impl Iterator<Item = u32> {
        (1..=9).filter(move |&v| self.possible & (1 << (v - 1)) != 0)
    }

    fn contains(&self, value: u32) -> bool {
        self.possible & (1 << (value - 1)) != 0
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            possible: 0b111_111_111,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sudoku {
    cells: [Cell; 81],
}

impl Default for Sudoku {
    fn default() -> Self {
        let cells = [Cell::default(); 81];
        Self { cells }
    }
}

impl Sudoku {
    pub fn parse(input: &str) -> Self {
        let mut result = Self::default();

        let input: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

        for i in 0..81usize {
            if let Some(value) = input[i].to_digit(10) {
                result.fill(i, value);
            }
        }

        result
    }

    pub fn is_valid(&self) -> bool {
        let mut seen = HashSet::new();

        for r in 0..9 {
            seen.clear();

            for c in 0..9 {
                if let Some(v) = self.cells[r * 9 + c].is_solved() {
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
                if let Some(v) = self.cells[r * 9 + c].is_solved() {
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
                        if let Some(v) = self.cells[idx].is_solved() {
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

    pub fn solve(&self) -> Option<Self> {
        let best_idx = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.count() > 1)
            .min_by_key(|(_, cell)| cell.count())
            .map(|(i, _)| i);

        if let Some(i) = best_idx {
            for p in self.cells[i].possible_values() {
                let mut next = self.clone();

                if !next.fill(i, p) {
                    continue;
                }

                if let Some(s) = next.solve() {
                    return Some(s);
                }
            }

            None
        } else {
            Some(self.clone())
        }
    }

    fn fill(&mut self, i: usize, value: u32) -> bool {
        for n in 1..=9 {
            if n == value {
                continue;
            }

            if !self.eliminate(i, n) {
                return false;
            }
        }

        true
    }

    fn eliminate(&mut self, i: usize, value: u32) -> bool {
        if !self.cells[i].contains(value) {
            return true;
        }

        self.cells[i].remove(value);
        if self.cells[i].count() == 0 {
            return false;
        }

        if let Some(v) = self.cells[i].is_solved() {
            for j in peers::PEERS[i] {
                if !self.eliminate(j, v) {
                    return false;
                }
            }
        }

        if !self.handle_neighbors(&peers::UNITS[i][..9], value) {
            return false;
        }

        if !self.handle_neighbors(&peers::UNITS[i][9..18], value) {
            return false;
        }

        if !self.handle_neighbors(&peers::UNITS[i][18..], value) {
            return false;
        }

        true
    }

    fn handle_neighbors(&mut self, nbr: &[usize], value: u32) -> bool {
        let mut count = 0;
        let mut found = usize::MAX;

        for &i in nbr {
            if self.cells[i].contains(value) {
                count += 1;
                found = i;
            }
        }

        if count == 0 {
            return false;
        }

        if count == 1 && !self.fill(found, value) {
            return false;
        }

        true
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..9usize {
            for c in 0..9 {
                let idx = r * 9 + c;
                match self.cells[idx].is_solved() {
                    Some(v) => v.fmt(f)?,
                    None => write!(f, ".")?,
                }
            }

            writeln!(f)?
        }

        Ok(())
    }
}
