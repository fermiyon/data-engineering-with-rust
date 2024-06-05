// Basic Rayon parallel map example
use rayon::prelude::*;

fn rayon_example() {
    let vals = vec![1, 2, 3];
    let squared = vals
        .par_iter() // Rayon parallel iterator
        .map(|x| x * x)
        .collect::<Vec<_>>();

    println!("{:?}", squared);
}

// a function that finds out the average of several numbers and returns it
fn average(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    for number in numbers {
        sum += number;
    }
    // do a loop so that it prints out numbers
    for number in numbers {
        println!("{}", number);
    }
    sum / numbers.len() as f64
}

// main function
// this is the entry point of the program
fn main() {
    // create a numbers list
    let result = project::add(1, 2);
    println!("{}", result);
    rayon_example()
}
