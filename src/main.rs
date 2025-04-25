use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Print the initial messages
    println!("Guess the number!");
    println!("{:?}", "Guess the Number!"); // Similar to console.log

    // Generate a secret number between 1 and 100
    let secret_number: u32 = rand::thread_rng().random_range(1..=100);
    println!("The secret number is: {}", secret_number);

    // Prompt the user for their guess before reading input
    println!("Please input your guess.");

    // Create a mutable String variable to store the input
    let mut guess = String::new();

    // Read the user's guess into the 'guess' variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Parse the input (trimming whitespace) into a u32 number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Show the parsed guess (uses the number now, not the string)
    println!("You guessed: {}", guess);

    // Compare the guessed number with the secret number using match
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}