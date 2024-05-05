// Given a string of words, implement a function that returns the shortest word in the string.

use std::io;

fn find_shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let sentence = input.trim();

    if let Some(shortest_word) = find_shortest_word(&sentence) {
        println!("The shortest word in the string is: {}", shortest_word);
    } else {
        println!("No words found in the string.");
    }
}

