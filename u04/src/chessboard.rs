use crate::grids;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ChessBoard([[char; 8]; 8]);

impl ChessBoard {
    pub fn new(layout: [[char; 8]; 8]) -> Self {
        Self(layout)
    }

    pub fn blank() -> Self {
        Self([[' '; 8]; 8])
    }

    pub fn at_start() -> Self {
        Self([
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ])
    }

    pub fn reset(&mut self) {
        self.0[0] = ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'];
        self.0[1].fill('p');
        for row in &mut self.0[2..6] {
            row.fill(' ');
        }
        self.0[6].fill('P');
        self.0[7] = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(grids::format_grid_to(f, &self.0)?)
    }
}
