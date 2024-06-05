/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/
// This line imports the SliceRandom trait from the rand crate,
// which provides the shuffle method for randomizing elements in a slice.
use rand::seq::SliceRandom;
// This line imports the thread_rng function from the rand crate,
// which provides a thread-safe random number generator.
use rand::thread_rng;

fn main() {
    // This line creates a vector of strings called fruits.
    let mut fruits = vec!["apples", "bananas", "oranges", "pears"];
    // This line calls the shuffle method on the fruits vector,
    fruits.shuffle(&mut thread_rng());

    println!("Here is your fruit salad: {:?}", fruits);
}
