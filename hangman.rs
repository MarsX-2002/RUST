use std::io::{self, Write};
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();

    let words = vec![
        "rust",
        "programming",
        "compiler",
        "memory",
        "ownership",
        "borrowing",
        "lifetimes",
        "traits",
        "structs",
        "enums",
        "patterns",
    ];
    let word = words.choose(&mut rng).unwrap();

    let mut guessed_letters = vec!['_'; word.len()];
    let mut remaining_guesses = 6;
    let mut guessed = String::new();

    loop {
        print_status(&guessed_letters, remaining_guesses);

        if guessed_letters.iter().all(|&c| c != '_') {
            println!("Congratulations! You guessed the word.");
            break;
        }

        if remaining_guesses == 0 {
            println!("Game over! The word was {}.", word);
            break;
        }

        let guess = get_guess(&guessed);
        guessed.push(guess);

        if word.contains(guess) {
            println!("Correct guess!");
            for (i, c) in word.chars().enumerate() {
                if c == guess {
                    guessed_letters[i] = c;
                }
            }
        } else {
            println!("Wrong guess.");
            remaining_guesses -= 1;
        }
    }
}

fn print_status(guessed_letters: &[char], remaining_guesses: u8) {
    println!();
    println!("Word: {}", guessed_letters.iter().collect::<String>());
    println!("Guessed letters: {}", guessed_letters.iter().filter(|&&c| c != '_').collect::<String>());
    println!("Remaining guesses: {}", remaining_guesses);
}

fn get_guess(guessed: &str) -> char {
    let mut guess = String::new();

    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.len() != 1 {
            println!("Please enter a single letter.");
        } else if guessed.contains(guess) {
            println!("You already guessed that letter. Try again.");
        } else {
            return guess.chars().next().unwrap();
        }

        guess.clear();
    }
}
