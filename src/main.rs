use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Loop until the user guesses the number
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read the user input and return an error if it fail to read the line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the user input to a number and return an error if it fail to parse the number
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        // Compare the user input with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("The secret number is: {secret_number}");
                break;
            }
        }
    }
}
