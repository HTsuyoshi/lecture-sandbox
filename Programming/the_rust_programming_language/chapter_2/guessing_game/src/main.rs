use std::io; // IO library, this library comes from Standard Library or std
use rand::Rng; // Add rng methods to our scope
use std::cmp::Ordering; // Call the Less, Greater and Equal functions

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng() // Give us a particular random number generator
        .gen_range(1..101); // 1..101 is equal as 1..=100

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input yout guess:");

        let mut guess; // Variables are immutables by default, but we can let variables mutables using the keyword mut
        guess = String::new(); // String::new() -> new() is acossiated function of String

        io::stdin() // If we dont import std::io we can use std::io::stdin() to handle input from terminal
            .read_line(&mut guess) // Append the input of stdin without overwriting its content. References are imuttable by default
            .expect("Failed to read line"); // Handle error of io:Result object

        /*
        let guess: u32 = guess // Using Shadowing to reuse the guess variable
            .trim() // Remove white spaces or \n or \r etc...
            .parse() // Will only work on characters that can logicall be converted into numbers and so can easily cause errord
            .expect("Please type a number!");
        */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess); // Print the guess variable

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
