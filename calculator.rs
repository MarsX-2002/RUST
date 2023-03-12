use std::io;

fn main() {
    let mut input = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read input.");

    println!("Enter second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read input.");

    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read input.");

    let num1: f64 = num1.trim().parse().expect("Invalid input.");
    let num2: f64 = num2.trim().parse().expect("Invalid input.");

    match operator.trim() {
        "+" => println!("{} + {} = {}", num1, num2, num1 + num2),
        "-" => println!("{} - {} = {}", num1, num2, num1 - num2),
        "*" => println!("{} * {} = {}", num1, num2, num1 * num2),
        "/" => println!("{} / {} = {}", num1, num2, num1 / num2),
        _ => println!("Invalid operator."),
    }
}
