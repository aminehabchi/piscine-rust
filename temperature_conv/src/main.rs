use temperature_conv::*;

fn main() {
    println!("{} F = {} C", 0.0, fahrenheit_to_celsius(0.0));
    println!("{} F = {} C", 20, fahrenheit_to_celsius(20.0));
    println!("{} F = {} C", -1, fahrenheit_to_celsius(-1.0));

    println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
    println!("{} C = {} F", 7.0, celsius_to_fahrenheit(7.0));
    println!("{} C = {} F", 200.0, celsius_to_fahrenheit(200.0));

}