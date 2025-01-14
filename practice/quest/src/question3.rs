use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Input the first integer: ");
    io::stdin().read_line(&mut input1).unwrap();
    let first_integer: i32 = input1.trim().parse().unwrap();

    println!("Input the second integer: ");
    io::stdin().read_line(&mut input2).unwrap();
    let second_integer: i32 = input2.trim().parse().unwrap();

    let sum = first_integer + second_integer;

    println!("Sum of the above two integers = {}", sum);
}
