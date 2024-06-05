// Vector - Growable array, similar to Python list
// A Function of a vector example in rust
pub fn vector_example() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    // Print v
    println!("Vector: {:?}", v);

    let first = &v[0];
    println!("The first element is: {}", first);
}

// VecDeque - Double ended queue, similar to Python deque
// A Function of a VecDeque example in rust
pub fn vecdeque_example() {
    use std::collections::VecDeque;
    let mut v = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_front(0);
    // Print v
    println!("VecDeque: {:?}", v);
}

// HashMap - Similar to Python dict
// A Function of a HashMap example in rust
// The HashMap is a collection of key-value pairs.
// HashMap does not maintain any order of the elements.
pub fn hashmap_example() {
    use std::collections::HashMap;
    let mut h = HashMap::new();
    h.insert("name", "John");
    h.insert("age", "30");
    // Print h
    println!("HashMap: {:?}", h);
}
