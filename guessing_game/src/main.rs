use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let secret_number = rand::rng().random_range(1..=100);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // By default in rust, you cannot redeclare variables, However, if needed, you can shadow them
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }

        println!("You guessed: {guess}");
    }
}
