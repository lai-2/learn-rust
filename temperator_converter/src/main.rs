use std::io;

fn main() {
    println!("Temperator converter:");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
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

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("invalid choice, please select 1 or 2.");
    }
}

fn celsius_to_fahrenheit() {
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

    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;

    println!("{:.2}°C is {:.2}°F", temp, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit: ");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input.");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter a number");
            return;
        }
    };

    let celsius = (temp - 32.0) * 5.0 / 9.0;

    println!("{:.2}°F is {:.2}°C", temp, celsius);
}
