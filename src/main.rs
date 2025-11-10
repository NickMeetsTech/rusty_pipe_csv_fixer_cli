// In src/main.rs
use std::env; // <-- Add this
use std::fs;
mod fixer;
use fixer::{CsvConfig, RowStatus};

fn main() {
    // --- 1. GET ARGUMENTS ---
    let args: Vec<String> = env::args().collect();

    // Check if we have at least 2 arguments (plus the program name)
    if args.len() < 3 {
        panic!("Not enough arguments! Usage: cargo run -- <input_file> <output_file>");
    }
    
    // We use [1] and [2] because [0] is the program's name
    // We borrow them (`&`) because `fs::read_to_string` takes a &str
    let input_filename = &args[1];
    let output_filename = &args[2];

    println!("Reading from: {}", input_filename);
    println!("Writing to: {}", output_filename);

    // --- 2. READ ---
    let file_contents = fs::read_to_string(input_filename)
        .expect("Failed to read the input file");

    let mut fixed_rows: Vec<&str> = Vec::new();

    let header = file_contents.lines().next().expect("File is empty");
    let expected_fields = header.split(',').count();
    println!("--- Header has {} fields ---", expected_fields);

    // --- 3. PROCESS ---
    for (i, row) in file_contents.lines().enumerate() {
        let row_num = (i + 1) as u32;
        let status = fixer::inspect_row(row_num, row, expected_fields);

        match status {
            RowStatus::Valid => {
                println!("Row {} is Valid. Keeping.", row_num);
                fixed_rows.push(row); 
            }
            RowStatus::Empty => {
                println!("Row {} is Empty. Discarding.", row_num);
            }
            RowStatus::Broken { row_num, reason } => {
                println!("Row {} is Broken. Discarding. Reason: {}", row_num, reason);
            }
        }
    }
    
    // --- 4. WRITE ---
    let output_data = fixed_rows.join("\n");
    
    fs::write(output_filename, output_data)
        .expect("Failed to write to output file");

    println!("--- Summary ---");
    println!("Processing complete. Fixed file saved to: {}", output_filename);
}
