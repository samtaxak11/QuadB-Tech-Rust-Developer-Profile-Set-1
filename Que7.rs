// Implement a function that returns the kth smallest element in a given array.

use std::io;

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
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

    println!("Enter the value of k:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let k: usize = input.trim().parse().expect("Invalid input");

    if let Some(smallest) = kth_smallest_element(&arr, k) {
        println!("The {}th smallest element in the array is: {}", k, smallest);
    } else {
        println!("Invalid value of k.");
    }
}

