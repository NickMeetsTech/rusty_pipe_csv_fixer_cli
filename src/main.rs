// In src/main.rs
mod fixer;

fn main() {
    // Let's pretend we have a few rows of data to check
    let rows_to_check = [
        "header1,header2,header3",
        "data1,data2,data3",
        "data1,,data3", // A row with a missing field
        "",             // An empty row
    ];
    
    // We use a `for` loop to check each one!
    // .iter().enumerate() is a standard way to get both the
    // index (0, 1, 2...) and the data in a loop.
    for (i, row) in rows_to_check.iter().enumerate() {
        // `i` is a `usize`, we add 1 to make it a 1-based row number
        fixer::inspect_row((i + 1) as u32, row);
    }
}
