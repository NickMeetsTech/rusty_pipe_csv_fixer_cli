// In src/fixer.rs

#[derive(Debug)] 
pub struct CsvConfig {
    pub input_file: String,
    pub output_file: String,
}

impl CsvConfig {
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
