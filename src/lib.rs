pub use colored::*;
pub use rand::{seq::IteratorRandom, Rng};
pub use std::fmt;
pub use std::fs;
pub use std::io::{self, Write};
pub use std::ops::RangeInclusive;
pub use std::time::Duration;

pub mod games;

struct Menu<T> {
    items: Vec<(String, T)>,
    prompt: String,
    error_prompt: String,
}

impl<T> Menu<T> {
    fn new(items: Vec<(String, T)>, prompt: String, error_prompt: String) -> Menu<T> {
        Menu {
            items,
            prompt,
            error_prompt,
        }
    }

    fn prompt(&self) -> Option<&T> {
        println!("{}", self.prompt);
        self.items.iter().enumerate().for_each(|(idx, (name, _))| {
            println!("[{}] {}", idx + 1, name);
        });
        print!("> ");
        io::stdout().flush().unwrap();

        match user_number() {
            Ok(choice) if (1..=self.items.len() as i32).contains(&choice) => {
                Some(&self.items[choice as usize - 1].1)
            }
            _ => {
                println!("{}", self.error_prompt);
                None
            }
        }
    }
}

pub enum Coin {
    Heads,
    Tails,
}

pub fn coin_flip() -> Coin {
    match random_number(1..=2) {
        1 => Coin::Heads,
        _ => Coin::Tails,
    }
}

pub fn first_char(string: &str) -> Option<char> {
    string.chars().next()
}

pub fn is_letter(c: char) -> bool {
    ('A'..='z').contains(&c)
}

pub fn random_word() -> String {
    let mut rng = rand::thread_rng();
    let filename = "src/assets/words.txt";
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    contents.lines().choose(&mut rng).unwrap().to_string()
}

pub fn random_number(range: RangeInclusive<i32>) -> i32 {
    rand::thread_rng().gen_range(range)
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

pub fn enclose(string: &str) -> String {
    format!(
        "╭─{0}─╮\n│ {1} │\n╰─{0}─╯",
        std::iter::repeat("─")
            .take(string.len())
            .collect::<String>(),
        string
    )
}

pub fn scroll_print(string: &str, delay: Duration) {
    for c in string.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        std::thread::sleep(delay);
    }
    println!();
}

pub fn loading_bar(segment: &str, length: usize, variance: usize) {
    for _ in 0..length {
        print!("{}", segment);
        io::stdout().flush().unwrap();

        let variance = random_number((0..=variance as i32));
        std::thread::sleep(std::time::Duration::from_millis(variance as u64));
    }
}

// pub fn write_file()
