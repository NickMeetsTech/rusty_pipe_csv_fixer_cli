// In src/main.rs
use std::fs; // <-- Add this
mod fixer;
use fixer::{CsvConfig, RowStatus};

fn main() {
    // 1. Read the file into one giant String
    let file_contents = fs::read_to_string("input.csv")
        .expect("Failed to read input.csv. Make sure it's in the root folder.");

    let mut valid_rows = 0;
    let mut broken_rows = 0;

    // 2. Get the expected field count from the *first* line
    // .lines() returns an iterator. .next() gets the first item from it.
    let header = file_contents.lines().next().expect("File is empty, no header row found!");
    let expected_fields = header.split(',').count();
    println!("--- Header has {} fields ---", expected_fields);

    // 3. Loop over *all* lines (including the header again)
    // The .lines() iterator works directly in a for loop!
    for (i, row) in file_contents.lines().enumerate() {
        let row_num = (i + 1) as u32;

        // Our fixer logic doesn't need to change at all!
        let status = fixer::inspect_row(row_num, row, expected_fields);

        match status {
            RowStatus::Valid => {
                println!("Row {} is Valid.", row_num);
                valid_rows += 1;
            }
            RowStatus::Empty => {
                println!("Row {} is Empty, skipping.", row_num);
            }
            RowStatus::Broken { row_num, reason } => {
                println!("Row {} is Broken! Reason: {}", row_num, reason);
                broken_rows += 1;
            }
        }
    }

    println!("--- Summary ---");
    // We can't use .len() on an iterator. Let's just count.
    println!("Total Valid: {}", valid_rows);
    println!("Total Broken: {}", broken_rows);
}
