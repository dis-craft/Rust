use std::io; // Importing the input/output library

fn main() {
    // Create a mutable string to store user input
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Prompt the user for the first integer
    println!("Input the first integer: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input"); // Read user input
    let num1: i32 = input1.trim().parse().expect("Please enter a valid integer"); // Convert to integer

    // Prompt the user for the second integer
    println!("Input the second integer: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input"); // Read user input
    let num2: i32 = input2.trim().parse().expect("Please enter a valid integer"); // Convert to integer

    // Calculate the sum
    let sum = num1 + num2;

    // Print the result
    println!("Sum of the above two integers = {}", sum);
}
