use std::io;

fn main() {
    println!("Please enter some text:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().unwrap_or(0);

    match num {
        1 => println!("You typed: 1"),
        2 => println!("You typed: 2"),
        _ => println!("You typed something else"),
    }
}