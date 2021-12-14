use crate::{games::*, *};

pub struct Hangman {
    word: String,
    max_guesses: usize,
    total_guesses: usize,
    letters_guessed: Vec<char>,
}

impl Hangman {
    pub fn new(word: String) -> Hangman {
        Hangman {
            word,
            max_guesses: 8,
            total_guesses: 0,
            letters_guessed: Vec::new(),
        }
    }

    //   ‖‾‾‾|
    //   ‖   O
    //   ‖  /|\
    //   ‖   |
    //   ‖  / \
    // --╨--
    fn format_hangman(&self) -> String {
        let head = match self.total_guesses {
            n if n == self.max_guesses => "   X",
            1.. => "   O",
            _ => "",
        };

        let torso = match self.total_guesses {
            2 => "   |",
            3 => "  /|",
            4.. => "  /|\\",
            _ => "",
        };

        let abdomen = match self.total_guesses {
            5.. => "   |",
            _ => "",
        };

        let legs = match self.total_guesses {
            6 => "  /",
            7.. => "  / \\",
            _ => "",
        };

        let top = String::from("  ‖‾‾‾|");
        let head_section = String::from("  ‖") + head;
        let torso_section = String::from("  ‖") + torso;
        let abdomen_section = String::from("  ‖") + abdomen;
        let leg_section = String::from("  ‖") + legs;
        let bottom = String::from("--╨--");

        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            top, head_section, torso_section, abdomen_section, leg_section, bottom
        )
    }

    fn blank_unguessed(&self) -> String {
        let mut blanks = String::new();
        for letter in self.word.chars() {
            if is_letter(letter) && !self.letters_guessed.contains(&letter.to_ascii_lowercase()) {
                blanks += "_ ";
            } else {
                blanks += &format!("{} ", letter);
            }
        }
        blanks
    }

    fn unique_letters(&self) -> Vec<char> {
        let mut unique = Vec::new();
        for letter in self.word.chars() {
            if !unique.contains(&letter) && is_letter(letter) {
                unique.push(letter.to_ascii_lowercase());
            }
        }
        unique
    }

    fn has_won(&self, letters: Vec<char>) -> bool {
        for letter in letters {
            if !self.letters_guessed.contains(&letter) {
                return false;
            }
        }
        true
    }
}

impl Game for Hangman {
    fn gameloop(&mut self) -> GameResult {
        while self.total_guesses < self.max_guesses {
            println!();
            println!("{}", self.format_hangman());
            println!("{}", self.blank_unguessed());
            println!(
                "Guessed: [{}]",
                self.letters_guessed
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            println!(
                "Guesses Remaining: {}",
                self.max_guesses - self.total_guesses
            );
            let letters_in_word = self.unique_letters();

            let mut letter: char;

            // prompt user until a valid guess is entered
            loop {
                // get input on the same line
                println!();
                print!("Guess a letter: ");
                io::stdout().flush().unwrap();

                let guess = user_string();
                letter = match first_char(&guess) {
                    Some(c) => c.to_ascii_lowercase(),
                    None => {
                        // println!("Please guess a letter.");
                        println!("{}", "Please guess a letter.".red());
                        continue;
                    }
                };

                // check if guess is a valid letter
                if !is_letter(letter) {
                    // println!("'{}' is not a letter!", guess);
                    println!("{}", format!("'{}' is not a letter!", guess).red());
                    continue;
                }

                // check if letter has already been guessed
                if self.letters_guessed.contains(&letter) {
                    println!("{}", format!("You already guessed '{}'!", letter).red());
                    continue;
                }

                // add letter to guessed
                self.letters_guessed.push(letter);

                break;
            }

            if letters_in_word.contains(&letter) {
                println!("{}", format!("'{}' is in the word!", letter).green());
            } else {
                self.total_guesses += 1;
                println!("{}", format!("'{}' is not in the word!", letter).red());
            }

            if self.has_won(letters_in_word) {
                println!();
                println!("{}", self.format_hangman());
                println!("{}", self.blank_unguessed());
                println!("{}", "You Win!".green());
                println!("The word was: '{}'", self.word);
                return GameResult {
                    winner: Winner::Player,
                    award: self.total_guesses as i32,
                };
            }
        }
        println!();
        println!("{}", self.format_hangman());
        println!("You Lose!");
        println!("The word was: {}", self.word);
        GameResult {
            winner: Winner::Amy,
            award: (self.max_guesses / 2) as i32,
        }
    }
}
