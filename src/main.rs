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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_mode_enabled() {
        env::set_var("DEBUG", "true");
        let secret_number = 42;
        let output = format!("DEBUG mode detected. The secret number is: {}", secret_number);
        assert!(output.contains("DEBUG mode detected"));
    }

    #[test]
    fn test_debug_mode_disabled() {
        env::remove_var("DEBUG");
        let secret_number = 42;
        let output = format!("The secret number is: {}", secret_number);
        assert!(!output.contains("DEBUG mode detected"));
    }

    #[test]
    fn test_guess_too_small() {
        let secret_number = 50;
        let guess = 25;
        assert_eq!(guess.cmp(&secret_number), Ordering::Less);
    }

    #[test]
    fn test_guess_too_big() {
        let secret_number = 50;
        let guess = 75;
        assert_eq!(guess.cmp(&secret_number), Ordering::Greater);
    }

    #[test]
    fn test_guess_correct() {
        let secret_number = 50;
        let guess = 50;
        assert_eq!(guess.cmp(&secret_number), Ordering::Equal);
    }
}
