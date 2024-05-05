// Reverse a string in Rust

use std::io;

fn main() {
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let reversed_string: String = input.trim().chars().rev().collect();
    println!("Reversed string: {}", reversed_string);
}

