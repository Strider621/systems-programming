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

fn number_analyzer(){

    fn is_even(n:i32) -> bool{
        n % 2 == 0
    }
    
    let numbers_arr = [1,2,3,5,10,13,15,25];

    
    for &num in numbers_arr.iter() {

        if num % 3 == 0 && num % 5 == 0 {
            println!("{} FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} Fizz", num);
        } else if num % 5 == 0 {
            println!("{} Buzz", num);
        } else {
            
            if is_even(num) {
                println!("{} is even", num);
            } else {
                println!("{} is odd", num);
            }
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers_arr.len() {
        sum += numbers_arr[i];
        i += 1;
    }
    println!("The sum of numbers in tha array is : {}", sum);

    let mut largest = numbers_arr[0];
    for &num in numbers_arr.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number in array is : {}", largest);


}

fn guessing_game() {
    let secret: i32 = 15; 
    let mut attempts = 0;

    fn check_guess(guess: i32, secret: i32) -> i32 {
        if guess == secret {
            0
        } else if guess > secret {
            1
        } else {
            -1
        }
    }

    let mut guess = 10; 
    loop {
        attempts += 1;
        guess += 1; 

        let result = check_guess(guess, secret);
        if result == 0 {
            println!("Correct! The secret number is {}.", secret);
            break;
        } else if result == 1 {
            println!("{} is too high.", guess);
        } else {
            println!("{} is too low.", guess);
        }
    }

    println!("It took {} attempts to guess the secret number.", attempts);
}

fn main() {
    //temp_converter();
    //number_analyzer();
    guessing_game();
}


