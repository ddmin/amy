use crate::{games::*, *};
use std::cmp::Ordering;

pub struct GuessingGame {
    number: usize,
    range: (usize, usize),
    max_guesses: usize,
    total_guesses: usize,
}

impl GuessingGame {
    pub fn new(lower: usize, upper: usize) -> GuessingGame {
        let number = random_number(lower as i32..=upper as i32) as usize;
        let max_guesses = ((1f64 / (upper - lower + 1) as f64).log(0.5) + 1f64) as usize;
        GuessingGame {
            number,
            range: (lower, upper),
            max_guesses,
            total_guesses: 0,
        }
    }
}

impl Game for GuessingGame {
    fn gameloop(&mut self, players: (Player, Player)) -> GameResult {
        println!();
        let (player, amy) = players;
        println!(
            "{}",
            format!("Guess the number ({} to {})", self.range.0, self.range.1).blue()
        );

        while self.total_guesses < self.max_guesses {
            println!();
            println!(
                "{}",
                format!("Remaining Tries: {}", self.max_guesses - self.total_guesses).blue()
            );
            print!("Guess a number: ");
            io::stdout().flush().unwrap();

            let guess: usize = match user_number() {
                Ok(n) => n.abs() as usize,
                Err(_) => {
                    println!("{}", "Invalid Guess.".red());
                    continue;
                }
            };

            println!();
            match guess.cmp(&self.number) {
                Ordering::Less => println!("{}", "Too Low!".red()),
                Ordering::Greater => println!("{}", "Too High!".red()),
                Ordering::Equal => {
                    println!(
                        "{}",
                        format!("You got it! The number was {}.", self.number).green()
                    );
                    return GameResult {
                        winner: player,
                        award: self.max_guesses as i32 / 2,
                    };
                }
            }

            self.total_guesses += 1;
        }

        println!();
        println!(
            "{}",
            format!("You Lose. The number was {}.", self.number).red()
        );

        GameResult {
            winner: amy,
            award: self.max_guesses as i32 / 2,
        }
    }
}
