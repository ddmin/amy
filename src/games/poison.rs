use crate::{games::*, *};

pub struct Poison {
    current_number: usize,
    target_number: usize,
    tracker: Vec<Turn>,
}

impl Poison {
    pub fn new() -> Poison {
        Poison {
            current_number: 0,
            target_number: 21,
            tracker: Vec::new(),
        }
    }

    fn format_numbers(&self) -> String {
        let mut amy_says = String::new();
        let mut player_says = String::new();
        for (number, turn) in self.tracker.iter().enumerate() {
            match turn {
                Turn::Amy => {
                    amy_says += &format!("{: >2} ", number + 1);
                    player_says += "   ";
                }
                Turn::Human => {
                    player_says += &format!("{: >2} ", number + 1);
                    amy_says += "   ";
                }
            }
        }
        format!("AMY: {}\nYOU: {}", amy_says, player_says)
    }
}

pub enum Turn {
    Amy,
    Human,
}

impl Turn {
    fn flip(&mut self) {
        *self = match self {
            Turn::Amy => Turn::Human,
            Turn::Human => Turn::Amy,
        }
    }
}

impl Game for Poison {
    fn gameloop(&mut self, players: &Players) -> GameResult {
        let (player, amy) = players.get_players();

        let mut turn = match coin_flip() {
            Coin::Heads => Turn::Amy,
            Coin::Tails => Turn::Human,
        };
        while self.current_number < self.target_number {
            println!();
            match turn {
                Turn::Amy => {
                    println!("{}", "AMY's Turn".blue());
                    let amt = random_number(1..=2) as usize;
                    let msg = match amt {
                        1 => {
                            self.tracker.push(Turn::Amy);
                            "AMY said 1 number."
                        }
                        _ => {
                            self.tracker.push(Turn::Amy);
                            self.tracker.push(Turn::Amy);
                            "AMY said 2 numbers."
                        }
                    };
                    self.current_number += amt;
                    println!("{}", msg.magenta());
                }
                Turn::Human => {
                    println!("{}", format!("{}'s Turn", player.name()).blue());
                    print!("{}", "Amount of numbers to say (1 or 2): ".yellow());
                    io::stdout().flush().unwrap();
                    let amt = match user_number() {
                        Ok(n) => n.abs() as usize,
                        Err(_) => {
                            println!("{}", "Please enter an amount.".red());
                            continue;
                        }
                    };
                    match amt {
                        1 => {
                            self.tracker.push(Turn::Human);
                            self.current_number += amt
                        }
                        2 => {
                            self.tracker.push(Turn::Human);
                            self.tracker.push(Turn::Human);
                            self.current_number += amt;
                        }
                        _ => {
                            println!("{}", "Please enter 1 or 2.".red());
                            continue;
                        }
                    }
                }
            };
            turn.flip();
            println!();
            println!("{}", self.format_numbers().white());
        }

        println!();
        match turn {
            Turn::Human => {
                println!(
                    "{}",
                    format!("You Win! AMY said {}.", self.current_number).green()
                );
                GameResult {
                    winner: Winner::Human,
                    award: 2,
                }
            }
            Turn::Amy => {
                println!("{}", "You Lose!".red());
                GameResult {
                    winner: Winner::Human,
                    award: 2,
                }
            }
        }
    }
}
