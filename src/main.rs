extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

use std::io;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);

    loop {
        println!("You have {} turns left", turns_left);
        display_progress(&letters);

        println!("Please enter a letter to guess: ");
        let user_char = read_user_input_character();

        // Exit if user enters an sterisk '*'
        if user_char == '*' {
            break;
        }

        // Updated the 'revealed' state of each letter if the user has guess a correct letter, at least one revelead is changed to true
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        // If they have guess incorrectly lose a turn
        if !at_least_one_revealed {
            turns_left -= 1;
        }

        // Check game progress
        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats, you won! The word was {}", selected_word);
                break;
            }
            GameProgress::Lost => {
                println!("\nSorry, you lost ;(");
                break;
            }
        }

    }

    println!("Selected word was {}", selected_word);
}

fn select_word() -> String {
    /* Open file */
    let mut file = File::open("words.txt").expect("Could not open file!");

    // Load file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Ane error has occured while reading the file");

    // get individual words
    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    // generate random index
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    // Create empty vector
    let mut letters: Vec<Letter> = Vec::new();

    // Wrap each character in a Letter Struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:"); // Example: Progress: _ a _ a _ y

    // Display appropriate character (letter or _) for each letter
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else{
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string)
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    // get user input
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*'; }
            }
        }
        Err(_) => { return '*'; }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    // Determine if all letters have been revealed
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    // If you have turns left and at least one is not revealed
    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;

}