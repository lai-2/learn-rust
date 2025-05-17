use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Available operations: +, -, *, /");
    println!("Enter your expression (eg, 5 + 3): ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    // println!("{}", tokens);

    if tokens.len() != 3 {
        println!("Invalid input, please follow the format: <number> <operator> <number>");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid first number");
            return;
        }
    };

    let num2: f64 = match tokens[2].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid second number");
            return;
        }
    };

    let operator = tokens[1];


    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Invalid operator, use +,-,*,/");
            return;
        }
    };

    println!("Result: {:.2} {:} {:.2} = {:.2}", num1, operator, num2, result);
}

fn add(num1: f64, num2: f64) -> f64 {
    return num1 + num2;
}

fn subtract(num1: f64, num2: f64) -> f64 {
    return num1 - num2;
}

fn multiply(num1: f64, num2: f64) -> f64 {
    return num1 * num2;
}

fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        println!("Devision by zero is not allowed");
        std::process::exit(1);
    }

    return num1 / num2;
}