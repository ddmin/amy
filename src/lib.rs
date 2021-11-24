pub use colored::*;
pub use rand::{seq::IteratorRandom, Rng};
pub use std::fs;
pub use std::io::{self, Write};

pub mod games;

fn first_char(string: &str) -> Option<char> {
    string.chars().next()
}

fn is_letter(c: char) -> bool {
    ('A'..='z').contains(&c)
}

pub fn random_word() -> String {
    let mut rng = rand::thread_rng();
    let filename = "src/assets/words.txt";
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    contents.lines().choose(&mut rng).unwrap().to_string()
}

pub fn random_number(lower: i32, upper: i32) -> i32 {
    rand::thread_rng().gen_range(lower..upper + 1)
}

pub fn user_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn user_number() -> Result<i32, std::num::ParseIntError> {
    user_string().parse()
}

// pub fn loading_bar()
// pub fn box_message()
// pub fn write_file()
