// In src/main.rs
mod fixer;

// We need to "use" the struct to bring it into scope
use fixer::CsvConfig;

fn main() {
    // Create an instance of our config
    let config = CsvConfig::new(
        String::from("input.csv"),
        String::from("output.csv"),
        ','
    );

    // Let's try to print it...
    println!("Config: {:#?}", config); // This will FAIL!
}
