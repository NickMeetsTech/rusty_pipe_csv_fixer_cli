// In src/main.rs
use std::env;
use std::error::Error;
// We no longer need fs, the csv crate handles file I/O
// We no longer need RowStatus

mod fixer;
use fixer::CsvConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = CsvConfig::new(&args)?;

    println!("Reading from: {}", config.input_file);
    println!("Writing to: {}", config.output_file);

    // Use the `csv` crate's Reader
    let mut reader = csv::Reader::from_path(&config.input_file)?;
    
    // Use the `csv` crate's Writer
    let mut writer = csv::Writer::from_path(&config.output_file)?;

    // 1. Get the headers
    // .headers() gives us a &StringRecord
    let headers = reader.headers()?.clone();
    let expected_fields = headers.len();
    
    // Write the headers to the output file first
    writer.write_record(&headers)?;

    println!("--- Header has {} fields ---", expected_fields);

    let mut valid_rows = 0;
    let mut broken_rows = 0;

    // 2. Iterate over each record
    // .records() gives us an iterator of Result<StringRecord>
    for (i, result) in reader.records().enumerate() {
        let row_num = (i + 2) as u32; // +1 for 0-index, +1 for header

        match result {
            Ok(record) => {
                // Check the field count
                if record.len() != expected_fields {
                    println!(
                        "Row {} is Broken. Reason: Mismatched field count. Expected {}, got {}",
                        row_num, expected_fields, record.len()
                    );
                    broken_rows += 1;
                    // Don't write this record
                } else {
                    // Row is valid, write it to the output file
                    writer.write_record(&record)?;
                    valid_rows += 1;
                }
            }
            Err(e) => {
                // This catches "jagged" rows or unquoted special characters
                println!("Row {} is Broken. Reason: {}", row_num, e);
                broken_rows += 1;
            }
        }
    }

    // 3. Finalize the write
    // .flush() ensures all data is written to the file
    writer.flush()?;

    println!("--- Summary ---");
    println!("Total Valid Rows: {}", valid_rows);
    println!("Total Broken Rows: {}", broken_rows);
    println!("Processing complete. Fixed file saved to: {}", config.output_file);

    Ok(())
}
