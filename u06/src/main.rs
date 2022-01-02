pub mod errors;
pub mod pointing;
pub mod sugrid;

use u06::sudoku::Sudoku;

fn main() {
    let mut s = Sudoku::new([
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]);
    println!("{}", s);
    // try an invalid digit
    if let Err(err) = s.set_cell(1, 1, 0) {
        eprintln!("{}", err);
    }
    // try an invalid cell
    if let Err(err) = s.set_cell(1, 10, 2) {
        eprintln!("{}", err);
    }
    // try a digit that's already in the same row
    if let Err(err) = s.set_cell(1, 1, 1) {
        eprintln!("{}", err);
    }
    // try a digit that's already in the same column
    if let Err(err) = s.set_cell(1, 1, 3) {
        eprintln!("{}", err);
    }
    // try a digit that's already in the same sub-grid
    if let Err(err) = s.set_cell(1, 1, 8) {
        eprintln!("{}", err);
    }
    // try to set a cell that's an initial value
    if let Err(err) = s.set_cell(3, 0, 9) {
        eprintln!("{}", err);
    }
    // try to clear a cell that's an initial value
    if let Err(err) = s.clear_cell(3, 0) {
        eprintln!("{}", err);
    }
    if s.set_cell(1, 1, 2).is_ok() {
        // 1[2]3[4]56[7]89
        println!("{}", s);
    }
}
