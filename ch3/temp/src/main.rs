// this program converts temperature from Celsius to Fahrenheit
// Formula: Fahrenheit = (Celsius * 1.8) + 32

use std::io;

fn main() {
    loop {
        println!("Please input a temperature in Celsius.");
        let mut celcius = String::new();

        io::stdin()
            .read_line(&mut celcius)
            .expect("Failed to read line.");

        let celcius: f32 = match celcius.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fahrenheit = (celcius * 1.8) + 32.0;
        println!("{celcius}°C is {fahrenheit}°F");
        break;
    }
}
