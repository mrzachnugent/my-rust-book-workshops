use std::io;

fn main() {
    exercise_converting_temperatures()
}

fn convert_fahrenheit_to_celsius(degree: f64) -> f64 {
    (degree - 32.0) * (5.0 / 9.0)
}

fn convert_celsius_to_fahrenheit(degree: f64) -> f64 {
    (degree * 9.0 / 5.0) + 32.0
}

fn exercise_converting_temperatures() {
    loop {
        let mut degree = String::new();
        println!("Enter a tempure value. Example: 27.6");
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line for tempurature value");

        let degree: f64 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut answer = String::new();
        println!("Do you want to convert your value to Celsius? Type 'y' for yes or any other value to convert the value to Fahrenheit");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line conversion type");
        println!("ANSWER: {}", answer.chars().next().unwrap());
        if answer.chars().next().unwrap() == 'y' && answer.len() == 2 {
            let conversion = convert_fahrenheit_to_celsius(degree);
            println!("{} degrees Celcius!", conversion);
            break;
        }

        let conversion = convert_celsius_to_fahrenheit(degree);
        println!("{} degrees Fahrenheit!", conversion);
        break;
    }
}
