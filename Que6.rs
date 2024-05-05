// Implement a function that finds the longest common prefix of a given set of strings.

use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first_string = &strings[0];
    let mut longest_prefix = String::new();
    
    'outer: for (i, ch) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(c) = string.chars().nth(i) {
                if c != ch {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(ch);
    }
    
    longest_prefix
}

fn main() {
    println!("Enter a set of strings separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let strings: Vec<String> = input.trim().split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let prefix = longest_common_prefix(&strings);
    if prefix.is_empty() {
        println!("There is no common prefix.");
    } else {
        println!("The longest common prefix is: {}", prefix);
    }
}

