pub fn print_table<T, U>(
    col_width: usize,
    headers: &[T],
    mut get_row: impl FnMut() -> Option<Vec<U>>,
) where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let rule_width = (col_width + 2) * headers.len() + headers.len() + 1;
    let rule = "=".repeat(rule_width);
    println!("{}", rule);
    let mut head = String::new();
    head.push('|');
    for col in headers {
        head.push_str(&format!(" {:width$} |", col.as_ref(), width = col_width));
    }
    println!("{}", head);
    println!("{}", rule);
    while let Some(cols) = get_row() {
        let mut row = String::new();
        row.push('|');
        for col in cols {
            row.push_str(&format!(" {:width$} |", col.as_ref(), width = col_width));
        }
        println!("{}", row);
    }
    println!("{}", rule);
}
