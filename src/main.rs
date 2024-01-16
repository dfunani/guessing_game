use rand::Rng;
use std::cmp::Ordering;
use std::{cmp, io};

fn main() {
    println!("Welcome to the Guessing Game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter a your guess");
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                println!("Secret Number: {secret_number}");
            }
            Err(err) => println!("{err}"),
        }
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Guess must be a number from 1 to 100");
                continue;
            }
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }
}
