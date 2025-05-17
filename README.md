# Course
[100 Days of Rust Development: Build a Project Every Day(AI)] (https://www.udemy.com/course/rust-programming-bootcamp)

# Project 1
[folder](./hello_rust/)

# Project 2
[folder](./temperator_converter)

## Parse number

```rs
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

## IF ELSE
```rs
if choice == 1 {
    celsius_to_fahrenheit();
} else if choice == 2 {
    fahrenheit_to_celsius();
} else {
    println!("invalid choice, please select 1 or 2.");
}
```

# Project 3
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

## Switch case
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
```
