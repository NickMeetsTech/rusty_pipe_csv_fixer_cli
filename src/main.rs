// In src/main.rs
mod fixer;
// Bring both CsvConfig and RowStatus into scope
use fixer::{CsvConfig, RowStatus}; 

fn main() {
    let rows_to_check = [
        "header1,header2,header3",
        "data1,data2,data3",
        "data1,,data3", // This will be "Broken"
        "",             // This will be "Empty"
    ];
    
    let mut valid_rows = 0;
    let mut broken_rows = 0;

    for (i, row) in rows_to_check.iter().enumerate() {
        let row_num = (i + 1) as u32;
        let status = fixer::inspect_row(row_num, row);

        // Use `match` to handle the returned enum
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
    println!("Total Valid: {}", valid_rows);
    println!("Total Broken: {}", broken_rows);
}
