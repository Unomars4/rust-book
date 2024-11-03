use std::io;

fn main() {
    println!("+++++++++++++++++++++++++++");
    println!("Guess the number!");
    println!("+++++++++++++++++++++++++++");

    println!("Please input your guess");

    let mut users_guess = String::new();

    io::stdin()
        .read_line(&mut users_guess)
        .expect("Failed to read the guess");

    println!("Your guess {}", users_guess);
}
