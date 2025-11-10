// In src/main.rs
use std::fs; // Make sure this is at the top
mod fixer;
use fixer::{CsvConfig, RowStatus};

fn main() {
    // --- 1. READ ---
    let file_contents = fs::read_to_string("input.csv")
        .expect("Failed to read input.csv");

    // This Vec will hold all the rows we want to KEEP
    let mut fixed_rows: Vec<&str> = Vec::new();

    let header = file_contents.lines().next().expect("File is empty");
    let expected_fields = header.split(',').count();
    println!("--- Header has {} fields ---", expected_fields);

    // --- 2. PROCESS ---
    for (i, row) in file_contents.lines().enumerate() {
        let row_num = (i + 1) as u32;
        let status = fixer::inspect_row(row_num, row, expected_fields);

        match status {
            RowStatus::Valid => {
                println!("Row {} is Valid. Keeping.", row_num);
                // Add the valid row to our Vec
                fixed_rows.push(row); 
            }
            RowStatus::Empty => {
                println!("Row {} is Empty. Discarding.", row_num);
            }
            RowStatus::Broken { row_num, reason } => {
                println!("Row {} is Broken. Discarding. Reason: {}", row_num, reason);
                // We could also try to "fix" it here, but for now
                // we'll just discard it.
            }
        }
    }
    
    // --- 3. WRITE ---
    
    // Join all the rows in our Vec into one string,
    // separating each one with a newline.
    let output_data = fixed_rows.join("\n");
    
    let output_filename = "output.csv";
    fs::write(output_filename, output_data)
        .expect("Failed to write to output.csv");

    println!("--- Summary ---");
    println!("Processing complete. Fixed file saved to: {}", output_filename);
}
