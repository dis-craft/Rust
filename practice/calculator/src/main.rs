use std::io;

fn main() {
    println!("Simple Calculator");

    // Input first number
    let num1 = get_input("Enter the first number: ");

    // Input operation
    println!("Choose an operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read input");
    let operation = operation.trim();

    // Input second number
    let num2 = get_input("Enter the second number: ");

    // Perform calculation
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operation! Please use +, -, *, or /.");
            return;
        }
    };

    println!("The result of {} {} {} is: {}", num1, operation, num2, result);
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse::<f64>().unwrap_or_else(|_| {
        println!("Invalid number! Please enter a valid numeric value.");
        std::process::exit(1);
    })
}
