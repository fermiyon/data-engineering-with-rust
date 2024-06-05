//! # Frequency Counter
//!
//! This module defines functionality to calculate the frequency of elements in a vector.
//! It provides a `logic` function that takes a vector of integers and returns a vector of tuples.
//! Each tuple contains an integer from the input vector and its corresponding frequency count.
//!
//! ## Usage
//!
//! The main function demonstrates how to use the `logic` function with a sample vector.
//! It prints the frequency of each number in a human-readable format.

use std::collections::HashMap;
use std::vec::Vec;

/// This function takes a vector of integers and returns a vector of tuples.

fn logic(numbers: Vec<i32>) -> Vec<(i32, i32)> {
    let mut frequencies = HashMap::new();
    for num in numbers {
        *frequencies.entry(num).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    for (key, value) in frequencies {
        result.push((key, value));
    }
    result
}
fn main() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 3, 4];
    let result = logic(input);

    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}
