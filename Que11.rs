// Merge two sorted arrays in Rust

use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    // Append remaining elements from arr1, if any
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    // Append remaining elements from arr2, if any
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

fn main() {
    println!("Enter the first sorted array of integers (space-separated):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
        .expect("Failed to read line");
    let arr1: Vec<i32> = input1.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the second sorted array of integers (space-separated):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
        .expect("Failed to read line");
    let arr2: Vec<i32> = input2.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged sorted array: {:?}", merged_array);
}

