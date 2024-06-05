/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit_salad = VecDeque::from(vec!["Apple", "Banana", "Orange"]);

    let mut salad: Vec<&str> = fruit_salad.into_iter().collect();

    salad.shuffle(&mut thread_rng());

    fruit_salad = salad.into_iter().collect();

    fruit_salad.push_front("Pomegranate");
    fruit_salad.push_back("Fig");
    fruit_salad.push_back("Cherry");

    println!("Fruit salad: {:?}", fruit_salad);
}
