use rand::{distributions::Bernoulli, prelude::Distribution, Rng};
use std::{fmt, iter};

pub struct ConwaySim {
    cur: Universe,
    next: Universe,
}

impl ConwaySim {
    pub fn new(start: Universe) -> Self {
        let next = start.clone();
        Self { cur: start, next }
    }

    pub fn next(&mut self) {
        self.cur.next_into(&mut self.next);
        std::mem::swap(&mut self.cur, &mut self.next);
    }

    pub fn universe<'a>(&'a self) -> &'a Universe {
        &self.next
    }
}

pub fn random(width: usize, height: usize) -> ConwaySim {
    ConwaySim::new(Universe::randomly_seeded(width, height))
}

pub fn from_seed(seed: Universe) -> ConwaySim {
    ConwaySim::new(seed)
}

#[derive(Debug, Clone)]
pub struct Universe {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    pub fn randomly_seeded(width: usize, height: usize) -> Self {
        let mut u = Self::new(width, height);
        u.seed(rand::thread_rng().gen_range(5..=70));
        u
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn seed(&mut self, percent: u32) {
        let mut rng = rand::thread_rng();
        let distribution = Bernoulli::from_ratio(percent.clamp(0, 100), 100).unwrap();
        for row in &mut self.cells {
            for cell in row {
                *cell = distribution.sample(&mut rng);
            }
        }
    }

    pub fn next_into(&self, next: &mut Universe) {
        assert!(self.dimensions() == next.dimensions());
        for (ri, (row, next_row)) in self.cells.iter().zip(next.cells.iter_mut()).enumerate() {
            for (ci, (cell, next_cell)) in row.iter().zip(next_row.iter_mut()).enumerate() {
                *next_cell = self.next_state(*cell, ri, ci);
            }
        }
    }

    fn next_state(&self, alive: bool, row: usize, col: usize) -> bool {
        match (alive, self.live_neighbors(row, col)) {
            (true, n) => n == 2 || n == 3,
            (false, 3) => true,
            (false, _) => false,
        }
    }

    fn live_neighbors(&self, row: usize, col: usize) -> usize {
        let (width, height) = (self.width, self.height);
        let prev_row = &self.cells[dec_index_wrapping(row, height)];
        let cur_row = &self.cells[row];
        let next_row = &self.cells[inc_index_wrapping(row, height)];
        let next_col = inc_index_wrapping(col, width);
        let prev_col = dec_index_wrapping(col, width);

        prev_row[prev_col] as usize
            + prev_row[col] as usize
            + prev_row[next_col] as usize
            + cur_row[prev_col] as usize
            + cur_row[next_col] as usize
            + next_row[prev_col] as usize
            + next_row[col] as usize
            + next_row[next_col] as usize
    }
}

#[inline]
fn inc_index_wrapping(index: usize, len: usize) -> usize {
    (index + 1) % len
}

#[inline]
fn dec_index_wrapping(index: usize, len: usize) -> usize {
    (index + len - 1) % len
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.width;
        let hmargin: String = iter::repeat('-').take(width).collect();
        writeln!(f, " {}", hmargin)?;
        for row in &self.cells {
            write!(f, "|")?;
            for cell in row.iter().copied() {
                write!(f, "{}", if cell { '+' } else { ' ' })?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, " {}", hmargin)?;
        Ok(())
    }
}
