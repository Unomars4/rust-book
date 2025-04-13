use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");
    let answer = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut users_guess = String::new();

        io::stdin()
            .read_line(&mut users_guess)
            .expect("Failed to read the guess");

        let users_guess: u32 = match users_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess {}", users_guess);

        match users_guess.cmp(&answer) {
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too high"),
            Ordering::Less => println!("Too low"),
        }
        println!("Try guessing again:")
    }
}
