use dotenv::dotenv;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::env;

fn main() {
    dotenv().ok();

    println!("\nGuess a number between 1 and 100!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    match env::var("DEBUG") {
        Ok(is_debug) if is_debug == "true" => {
            println!("DEBUG mode detected. The secret number is: {}", secret_number);
        }
        Ok(_) => {}
        Err(_) => eprintln!("DEBUG is not set in the environment."),
    }

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
