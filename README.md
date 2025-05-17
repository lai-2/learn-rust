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
