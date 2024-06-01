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
    let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("{}", average(&numbers));
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
}
