//! This module contains the core logic for the CSV fixer,
//! including configuration management.
/// Represents the configuration for the CSV fixer application.
///
/// Stores the input and output file paths provided by the user.
#[derive(Debug)] 
pub struct CsvConfig {
    /// The path to the input CSV file.
    pub input_file: String,
    /// The path to the output CSV file where fixed data will be written.
    pub output_file: String,
}

impl CsvConfig {
    /// Creates a new `CsvConfig` instance from user-provided arguments.
    ///
    /// This function parses a slice of `String` arguments. It expects
    /// at least two arguments (after the program name):
    /// 1. The input file path.
    /// 2. The output file path.
    ///
    /// # Arguments
    ///
    /// * `args`: A slice of strings, typically from `std::env::args()`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` with a static string message if there are not
    /// enough arguments.
    pub fn new(args: &[String]) -> Result<CsvConfig, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Usage: <input_file> <output_file>");
        }

        let input = args[1].clone();
        let output = args[2].clone();

        Ok(CsvConfig {
            input_file: input,
            output_file: output,
        })
    }
}

// This `cfg(test)` attribute tells Rust to only compile
// this module when we run `cargo test`.
#[cfg(test)]
mod tests {
    // We need to import the code we want to test
    use super::*;

    // The `#[test]` attribute marks this as a test function
    #[test]
    fn test_config_new_success() {
        // 1. Arrange: Set up our test data
        let args = vec![
            String::from("program_name"),
            String::from("input.csv"),
            String::from("output.csv"),
        ];

        // 2. Act: Run the function we're testing
        let config = CsvConfig::new(&args).unwrap(); // .unwrap() panics if it's Err

        // 3. Assert: Check that the result is correct
        assert_eq!(config.input_file, "input.csv");
        assert_eq!(config.output_file, "output.csv");
    }

    #[test]
    fn test_config_new_not_enough_args() {
        // 1. Arrange
        let args = vec![String::from("program_name")];

        // 2. Act
        let result = CsvConfig::new(&args);

        // 3. Assert
        // `is_err()` returns true if the Result is an Err
        assert!(result.is_err());
        
        // We can even check the error message
        assert_eq!(
            result.err().unwrap(),
            "Not enough arguments! Usage: <input_file> <output_file>"
        );
    }
}
