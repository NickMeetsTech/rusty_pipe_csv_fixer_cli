# Rusty Pipe - CSV Fixer CLI

**A high-performance CSV processing tool built in Rust.**

This project is a command-line application designed to read "broken" or inconsistent CSV files, filter out malformed rows, and produce a clean, valid output file.

This project was built as a capstone for learning the Rust programming language and to demonstrate core computer science principles for the **Northeastern University Align Computer Science Program**.

---

## Features

* **Robust Parsing:** Uses the official Rust `csv` crate to handle complex cases like quoted fields and escaped characters.
* **Validation:** Checks each row against the header's field count to identify and filter out broken rows.
* **Error Handling:** Built with Rust's `Result` type to handle file I/O and parsing errors gracefully.
* **Fully Tested:** Includes a unit test suite to verify core logic.
* **Documented:** The entire codebase is documented using `rustdoc`.

---

## Usage

To run the program, provide the input file path and the desired output file path as command-line arguments.

### Running from source (with Cargo)

```bash
# Example:
cargo run -- input.csv fixed_output.csv