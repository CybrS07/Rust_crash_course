use std::io; // Import the standard input/output library

fn main() {
    println!("Please enter some text:");

    // 1. Create a mutable String buffer to store the input
    let mut input = String::new();

    // 2. Read the line from standard input
    io::stdin()
        .read_line(&mut input) // Pass a mutable reference to the buffer
        .expect("Failed to read line"); // Handle potential errors

    // 3. Print the input (using .trim() to remove the trailing newline)
    println!("You typed: {}", input.trim());
}
