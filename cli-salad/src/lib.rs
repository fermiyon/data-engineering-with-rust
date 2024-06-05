use rand::{seq::SliceRandom, thread_rng};

// Function to create a fruit salad with a specified number of fruits
pub fn create_fruit_salad(num_fruits: usize) -> Result<Vec<&'static str>, String> {
    // Define an array of fruit names
    let fruits = [
        "Arbutus",
        "Loquat",
        "Strawberry Tree Berry",
        "Pomegranate",
        "Fig",
        "Cherry",
        "Orange",
        "Pear",
        "Peach",
        "Apple",
    ];

    // Check if the requested number of fruits is more than the available fruits
    if num_fruits > fruits.len() {
        return Err("Requested number of fruits exceeds the available fruits.".to_string());
    }

    // Create a mutable random number generator
    let mut rng = thread_rng();

    // Shuffle the array of fruits using the random number generator
    let mut shuffled_fruits = fruits;
    shuffled_fruits.shuffle(&mut rng);

    // Return a vector containing only the requested number of fruits
    Ok(shuffled_fruits[..num_fruits].to_vec())
}
