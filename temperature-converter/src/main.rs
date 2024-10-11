use std::io;

fn main() {
    println!("Please enter the temperature in Celsius:");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read input");

    let celsius: i32 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let temperature_in_fahrenheit: i32 = celsius * 9 / 5 + 32;

    println!("Temperature is:\nCelsius: {celsius}\nFahrenheit: {temperature_in_fahrenheit}");
}
