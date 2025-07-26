
fn temp_converter(){

    const freezePoint:f64 = 32.0;
    fn fahrenheit_to_celsius(f:f64) -> f64{
        (f - freezePoint) * (5.0/9.0)
    }

    fn celsius_to_fahrenheit(c: f64) -> f64{
        (c * (5.0/9.0)) + freezePoint

    }

    let mut fahrenheit = 72.0;

    fahrenheit_to_celsius(fahrenheit);
    println!{"{}F is {:.2}C in celsius.",fahrenheit, fahrenheit_to_celsius(fahrenheit)};

      for _ in 0..5 {
        fahrenheit += 1.0; // update the same mutable variable
        println!("{}F is {:.2}C in celsius", fahrenheit, fahrenheit_to_celsius(fahrenheit));
    }


}

fn main() {
    temp_converter();
}

// Create a Rust program that converts temperatures between Fahrenheit and Celsius. The program should:

// 1. Declare a constant for the freezing point of water in Fahrenheit (32°F).
// 2. Implement two functions:
//    - `fahrenheit_to_celsius(f: f64) -> f64`: Converts Fahrenheit to Celsius
//    - `celsius_to_fahrenheit(c: f64) -> f64`: Converts Celsius to Fahrenheit
// 3. In the main function:
//    - Declare a mutable variable with a temperature in Fahrenheit
//    - Convert it to Celsius using your function and print the result
//    - Use a loop to convert and print the next 5 integer temperatures (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)
