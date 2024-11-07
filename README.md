# Guessing Game

This is a simple command-line guessing game written in Rust. The program generates a random number between 1 and 100, and the user has to guess the number. If the `DEBUG` environment variable is set to `true`, the secret number will be displayed for debugging purposes.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)
- dotenv crate (for managing environment variables)

## Running the Game

1. Clone the repository or download the source code.
2. Navigate to the project directory.
3. Create a `.env` file in the project directory and add the following line if you want to enable debug mode:
    ```env
    DEBUG=true
    ```
4. Build and run the project using Cargo:
    ```sh
    cargo run
    ```

## How to Play

1. The program will prompt you to guess a number between 1 and 100.
2. Enter your guess and press Enter.
3. The program will tell you if your guess is too low, too high, or correct.
4. Keep guessing until you find the correct number.

## Example

```sh
$ cargo run

Guess a number between 1 and 100!

Please input your guess:
50
You guessed: 50
Too high!

Please input your guess:
25
You guessed: 25
Too low!

Please input your guess:
37
You guessed: 37
You win!
```