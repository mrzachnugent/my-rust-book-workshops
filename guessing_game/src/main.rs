//use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number! (in less than 10 tries...)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut number_of_guesses = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // std::io::stdin()
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        number_of_guesses += 1;

        println!(
            "You guessed: {} and this is try #{}",
            guess, number_of_guesses
        );

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        if number_of_guesses > 9 {
            println!("You lose! The correct number was {}.", secret_number );
            break;
        }
    }
}
