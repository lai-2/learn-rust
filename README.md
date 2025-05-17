# Course
[100 Days of Rust Development: Build a Project Every Day(AI)](https://www.udemy.com/course/rust-programming-bootcamp)

# Project 1: Hello world
[folder](./hello_rust/)

# Project 2: Temperator converter
[folder](./temperator_converter)

## Parse number

```rs
use std::io;

println!("Please select an option (1 or 2): ");
let mut choice = String::new();
io::stdin()
    .read_line(&mut choice)
    .expect("Failed to read input");

let choice: u32 = match choice.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("invalid choice, please enter 1 or 2.");
        return;
    }
};
```

```rs
println!("Enter temperaure in Celsius: ");

let mut temp = String::new();
io::stdin()
    .read_line(&mut temp)
    .expect("Failed to read input");

let temp: f64 = match temp.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("invalid input, please enter a number of temperatur");
        return;
    }
};
```

## if else
```rs
if choice == 1 {
    celsius_to_fahrenheit();
} else if choice == 2 {
    fahrenheit_to_celsius();
} else {
    println!("invalid choice, please select 1 or 2.");
}
```

# Project 3: Simple Calculator
[folder](./simple_calculator)

## String to list
```rs
let mut input = String::new();

io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input.");

let tokens: Vec<&str> = input.trim().split_whitespace().collect();

// get length of list
if tokens.len() != 3 {
    println!("Invalid input, please follow the format: <number> <operator> <number>");
    return;
}
```

## switch case
```rs
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
```

## function and exit
```rs
fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        println!("Devision by zero is not allowed");
        std::process::exit(1);
    }

    return num1 / num2;
}
```

# Project 4: Guessing Game
[folder](./guessing_game)
## Install package in Rust
1. Open file `Cargo.toml`, add line:
    ```toml
    [dependencies]
    rand = "0.8"
    ```
2. Install package:
    ```sh
    cargo build
    ```

## Random a number
```rs
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..=100);
```

## Loop
```rs
loop {
    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number.");
            continue;
        }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! Try again."),
        Ordering::Greater => println!("Too big! Try again."),
        Ordering::Equal => {
            println!("CONGRATULATIONS! You guessed the number!");
            break;
        }
    }
}
```

# Project 5: Word counter
[folder](./word_counter)

## Collect args from command-line
```rs
use std::env;

    let args: Vec<String> = env::args().collect();
```

## Read file content
```rs
use std::fs::File;
use std::io::{Read};

    // Read the file contents
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", err);
        return;
    }
```

# Project 6: BMI Calculator
[folder](./bmi_calculator/)

## read float number
```rs

fn main() {
    println!("Enter your weight in kilograms (kg): ");

    let weight = match get_input_as_f64() {
        Some(value) => value,
        None => {
            eprintln!("Invalid input for weight. Please enter a valid number.");
            return;
        }
    };
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
```