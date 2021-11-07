extern crate rand;
use rand::Rng;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let selected_word = select_word();

    println!("The selected word was {}", selected_word);
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