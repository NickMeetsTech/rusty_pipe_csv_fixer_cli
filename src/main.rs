// In src/main.rs
mod fixer;
use fixer::{CsvConfig, RowStatus};

fn main() {
    let rows_to_check = vec![
        "header1,header2,header3", // Expected: 3
        "data1,data2,data3",       // 3 -> Valid
        "data1,,data3",            // 3 -> Valid
        "data1,data2",             // 2 -> Broken!
        "",                        // 0 -> Empty!
        "data1,data2,data3,data4", // 4 -> Broken!
    ];

    let mut valid_rows = 0;
    let mut broken_rows = 0;

    // 1. Get the expected field count from the header
    // We use .get(0) to safely access the first element
    let header = rows_to_check.get(0).expect("No header row found!");
    let expected_fields = header.split(',').count();
    println!("--- Header has {} fields ---", expected_fields);

    // 2. Loop and inspect the rows
    // .iter().enumerate() gives us (index, &data)
    for (i, row) in rows_to_check.iter().enumerate() {
        let row_num = (i + 1) as u32;

        // Pass the expected_fields count to our function
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
    println!("Total Rows Checked: {}", rows_to_check.len());
    println!("Total Valid: {}", valid_rows);
    println!("Total Broken: {}", broken_rows);
}
