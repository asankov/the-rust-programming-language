use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {}", secret_number);

    loop {
        println!("Guess a number: ");

        let mut guess = String::new();
        io::stdin()
            // pass a reference and indicate that it's mutable so that the read_line method can mutate
            // it
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.eq("quit") {
            break;
        }

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("Plase input a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }
}
