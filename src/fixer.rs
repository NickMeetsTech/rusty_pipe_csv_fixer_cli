// In src/fixer.rs

pub fn fix_csv() {
    println!("Calling the fix_csv function!");
}

// NEW FUNCTION
pub fn inspect_row(row_number: u32, row_data: &str) {
    println!("Checking row {}: '{}'", row_number, row_data);

    if row_data.is_empty() {
        println!("  -> Result: This row is EMPTY.");
    } else if row_data.len() < 10 {
        // .len() returns the length in bytes
        println!("  -> Result: This row is very short. (<10 bytes)");
    } else {
        println!("  -> Result: This row looks okay for now.");
    }
}
