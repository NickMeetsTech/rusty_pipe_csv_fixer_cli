use std::env;
use std::fs;
use std::error::Error; // <-- Import the Error trait

mod fixer;
use fixer::{CsvConfig, RowStatus};

// 1. Change the signature of main
fn main() -> Result<(), Box<dyn Error>> {
    // --- 1. GET ARGUMENTS ---
    let args: Vec<String> = env::args().collect();

    // 2. Use `?` to handle the result from CsvConfig::new
    let config = CsvConfig::new(&args)?;

    println!("Reading from: {}", config.input_file);
    println!("Writing to: {}", config.output_file);

    // --- 2. READ ---
    // 3. Use `?` instead of .expect()
    let file_contents = fs::read_to_string(&config.input_file)?;

    let mut fixed_rows: Vec<&str> = Vec::new();

    // 4. Handle the header with `?` as well (using .ok_or())
    let header = file_contents.lines()
        .next()
        .ok_or("File is empty, no header row found!")?; // Converts Option to Result
        
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
    
    // 5. Use `?` instead of .expect()
    fs::write(&config.output_file, output_data)?;

    println!("--- Summary ---");
    println!("Processing complete. Fixed file saved to: {}", config.output_file);

    // 6. Return Ok(()) on success
    Ok(())
}
