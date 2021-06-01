use std::fmt;

pub fn format_grid_to<W, T, const R: usize, const C: usize>(
    w: &mut W,
    grid: &[[T; C]; R],
) -> fmt::Result
where
    W: fmt::Write,
    T: fmt::Display,
{
    for row in grid {
        for cell in row {
            write!(w, "[{}]", cell)?;
        }
        writeln!(w)?;
    }
    Ok(())
}

pub fn format_vec_grid_to<W, T>(w: &mut W, grid: &Vec<Vec<T>>) -> fmt::Result
where
    W: fmt::Write,
    T: fmt::Display,
{
    for row in grid {
        for cell in row {
            write!(w, "[{}]", cell)?;
        }
        writeln!(w)?;
    }
    Ok(())
}
