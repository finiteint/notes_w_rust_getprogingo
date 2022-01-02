use std::fmt;

const SIZE: usize = 9;
const SUB_SIZE: usize = 3;
const EMPTY: u8 = 0;

type Grid = [[u8; SIZE]; SIZE];

#[derive(Debug, Clone)]
pub struct Sudoku {
    initial: Grid,
    grid: Grid,
}

#[derive(Debug, thiserror::Error)]
pub enum SudokuError {
    #[error("invalid cell at ({0}, {1})")]
    InvalidCell(usize, usize),
    #[error("invalid digit {0}")]
    InvalidDigit(u8),
    #[error("digit {0} already exists in row {1}")]
    DigitAlreadyExistsInRow(u8, usize),
    #[error("digit {0} already exists in column {1}")]
    DigitAlreadyExistsInColumn(u8, usize),
    #[error("digit {0} already exists in subgrid")]
    DigitAlreadyExistsInSubGrid(u8),
    #[error("initial valued cell at ({0}, {1}) is not modifiable")]
    InitialValuedCell(usize, usize),
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            initial: [[EMPTY; SIZE]; SIZE],
            grid: [[EMPTY; SIZE]; SIZE],
        }
    }
}

impl Sudoku {
    pub fn new(initial: [[u8; SIZE]; SIZE]) -> Self {
        assert!(initial
            .iter()
            .enumerate()
            .flat_map(|(r, rv)| rv.iter().enumerate().map(move |(c, d)| (r, c, *d)))
            .all(|(r, c, d)| d == EMPTY
                || (is_valid_digit(d)
                    && !has_digit_in_column(&initial, r, c, d)
                    && !has_digit_in_row(&initial, r, c, d))));
        // bug: can introduce invalid values; some constraints are not checked
        let grid = initial;
        Self { initial, grid }
    }

    pub fn reset_to_initial(&mut self) {
        self.grid.clone_from(&self.initial);
    }

    pub fn set_cell(&mut self, row: usize, col: usize, digit: u8) -> Result<(), SudokuError> {
        if !is_in_grid(row, col) {
            return Err(SudokuError::InvalidCell(row, col));
        }
        if !is_valid_digit(digit) {
            return Err(SudokuError::InvalidDigit(digit));
        }
        if self.is_initial_valued_cell(row, col) {
            return Err(SudokuError::InitialValuedCell(row, col));
        }
        if has_digit_in_row(&self.grid, row, col, digit) {
            return Err(SudokuError::DigitAlreadyExistsInRow(digit, row));
        }
        if has_digit_in_column(&self.grid, row, col, digit) {
            return Err(SudokuError::DigitAlreadyExistsInColumn(digit, col));
        }
        if has_digit_in_subgrid(&self.grid, row, col, digit) {
            return Err(SudokuError::DigitAlreadyExistsInSubGrid(digit));
        }

        self.grid[row][col] = digit;
        Ok(())
    }

    pub fn clear_cell(&mut self, row: usize, col: usize) -> Result<(), SudokuError> {
        if !is_in_grid(row, col) {
            return Err(SudokuError::InvalidCell(row, col));
        }
        if self.is_initial_valued_cell(row, col) {
            return Err(SudokuError::InitialValuedCell(row, col));
        }

        self.grid[row][col] = EMPTY;
        Ok(())
    }

    #[inline]
    fn is_initial_valued_cell(&self, row: usize, col: usize) -> bool {
        self.initial[row][col] != EMPTY
    }
}

#[inline]
fn has_digit_in_subgrid(grid: &Grid, row: usize, col: usize, digit: u8) -> bool {
    let (sub_row, sub_col) = (row % SUB_SIZE, col % SUB_SIZE);
    let (sub_row_0, sub_col_0) = (row / SUB_SIZE, col / SUB_SIZE);
    grid[sub_row_0..sub_row_0 + SUB_SIZE]
        .iter()
        .enumerate()
        .flat_map(|(sr, sds)| {
            sds[sub_col_0..sub_col_0 + SUB_SIZE]
                .iter()
                .enumerate()
                .map(move |(sc, sd)| (sr, sc, *sd))
        })
        .any(|(r, c, d)| d != EMPTY && r != sub_row && c != sub_col && d == digit)
}

#[inline]
fn has_digit_in_row(grid: &Grid, row: usize, col: usize, digit: u8) -> bool {
    grid[row]
        .iter()
        .enumerate()
        .any(|(i, v)| i != col && *v == digit)
}

#[inline]
fn has_digit_in_column(grid: &Grid, row: usize, col: usize, digit: u8) -> bool {
    (0..SIZE).any(|r| r != row && grid[r][col] == digit)
}

#[inline]
fn is_in_grid(row: usize, column: usize) -> bool {
    row < SIZE && column < SIZE
}

#[inline]
fn is_valid_digit(digit: u8) -> bool {
    (1..=9).contains(&digit) // 1 <= digit && digit <= 9
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn column_numbers(f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "    ")?;
            for col_num in 1..=SIZE {
                write!(f, " {} ", col_num)?;
                if col_num % SUB_SIZE == 0 {
                    write!(f, "  ")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)
        }
        column_numbers(f)?;
        writeln!(f, "    -----------  -----------  -----------")?;
        for (row_num, row) in (1..=SIZE).zip(self.grid.iter()) {
            write!(f, "{:<2}::", row_num)?;
            for (col_num, cell) in (1..=SIZE).zip(row) {
                if *cell == EMPTY {
                    write!(f, "   ")?;
                } else {
                    write!(f, " {} ", cell)?;
                }
                if col_num % SUB_SIZE == 0 {
                    write!(f, "::")?;
                } else {
                    write!(f, "|")?;
                }
            }
            writeln!(f, "{:2}", row_num)?;
            if row_num % SUB_SIZE == 0 {
                writeln!(f, "    -----------  -----------  -----------")?;
            }
        }
        column_numbers(f)?;
        Ok(())
    }
}
