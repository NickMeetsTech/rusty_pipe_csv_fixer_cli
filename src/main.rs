// In src/main.rs
mod fixer;

fn main() {
    println!("--- Running Normalizer ---");

    // Create a mutable, owned String
    let mut row1 = String::from("data1,DATA2,data3");
    
    println!("Before: {}", row1);

    // Pass a mutable borrow to our fixer
    fixer::normalize_row(&mut row1);

    println!("After:  {}", row1);
    println!("--------------------------");
}

