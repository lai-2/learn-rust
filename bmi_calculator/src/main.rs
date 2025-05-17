use std::io;

fn main() {
    println!("BMI Calculator");
    println!("Enter your weight in kilograms (kg): ");

    let weight = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for weight. Please enter a valid number.");
            return;
        }
    };

    println!("Enter your height in meters (m): ");

    let height = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for height. Please enter a valid number.");
            return;
        }
    };

    if height == 0.0 {
        println!("Height cannot be zero.");
        return;
    }

    let bmi = calculate_bmi(weight, height);
    let category = classify_bmi(bmi);

    println!("Your BMI is {:.2}", bmi);
    println!("BMI Category: {}", category);
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

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    return weight / (height * height);
}

fn classify_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        return "Underweight";
    } else if bmi >= 18.5 && bmi <= 24.9 {
        return "Normal weight";
    } else if bmi >= 25.0 && bmi <= 29.9 {
        return "Overweight";
    } else {
        return "Obesity";
    }
}
