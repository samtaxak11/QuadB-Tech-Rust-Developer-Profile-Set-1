// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    
    while low <= high {
        let mid = low + (high - low) / 2;
        
        if arr[mid] == target && (mid == 0 || arr[mid - 1] != target) {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    
    None
}

fn main() {
    println!("Enter a sorted array of integers (space-separated):");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the target number:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let target: i32 = input.trim().parse().expect("Invalid input");

    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array.", target);
    }
}

