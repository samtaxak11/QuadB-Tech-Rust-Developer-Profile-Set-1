// Given a sorted array of integers, implement a function that returns the median of the array.

use std::io;

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        let mid_val1 = arr[mid - 1];
        let mid_val2 = arr[mid];
        return (mid_val1 as f64 + mid_val2 as f64) / 2.0;
    } else {
        return arr[len / 2] as f64;
    }
}

fn main() {
    println!("Enter a sorted array of integers (space-separated):");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let median = find_median(&arr);
    println!("The median of the array is: {}", median);
}

