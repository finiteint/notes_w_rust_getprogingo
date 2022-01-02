use anyhow::anyhow;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Grid {
    data: Vec<Vec<u8>>,
    rows: usize,
    columns: usize,
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum EnumeratedSudokuError {
    #[error("invalid cell index")]
    InvalidCellIndex,
    #[error("invalid digit {0} (expected 1..=9)")]
    InvalidDigit(u8),
}

#[derive(Debug, thiserror::Error)]
pub struct CollectedSudokuError(Vec<EnumeratedSudokuError>);

impl CollectedSudokuError {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn errors(&self) -> &[EnumeratedSudokuError] {
        &self.0
    }
}

impl fmt::Display for CollectedSudokuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, err) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            data: vec![vec![0u8; columns]; rows],
        }
    }

    pub fn set_with_basic_error(
        &mut self,
        row: usize,
        column: usize,
        digit: u8,
    ) -> Result<(), anyhow::Error> {
        if !self.is_in_grid(row, column) {
            return Err(anyhow!("({}, {}) is outside the grid", row, column));
        }

        if !self.is_valid_digit(digit) {
            return Err(anyhow!("invalid digit `{}`", digit));
        }

        self.data[row][column] = digit;
        Ok(())
    }

    pub fn set_with_enumerated_error(
        &mut self,
        row: usize,
        column: usize,
        digit: u8,
    ) -> Result<(), EnumeratedSudokuError> {
        if !self.is_in_grid(row, column) {
            return Err(EnumeratedSudokuError::InvalidCellIndex);
        }

        if !self.is_valid_digit(digit) {
            return Err(EnumeratedSudokuError::InvalidDigit(digit));
        }

        self.data[row][column] = digit;
        Ok(())
    }

    pub fn set_with_collected_error(
        &mut self,
        row: usize,
        column: usize,
        digit: u8,
    ) -> Result<(), anyhow::Error> {
        let mut errors = CollectedSudokuError::new();
        if !self.is_in_grid(row, column) {
            errors.0.push(EnumeratedSudokuError::InvalidCellIndex);
        }

        if !self.is_valid_digit(digit) {
            errors.0.push(EnumeratedSudokuError::InvalidDigit(digit));
        }

        if !errors.0.is_empty() {
            return Err(errors.into());
        }

        self.data[row][column] = digit;
        Ok(())
    }

    #[inline]
    fn is_in_grid(&self, row: usize, column: usize) -> bool {
        row < self.rows && column < self.columns
    }

    #[inline]
    fn is_valid_digit(&self, digit: u8) -> bool {
        0 < digit && digit < 10
    }
}
