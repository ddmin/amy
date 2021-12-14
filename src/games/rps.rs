use crate::{games::*, *};

pub struct RpsGame {
    best_of: usize,
    played: usize,
    score: Score,
}

pub enum RpsWinner {
    Amy,
    Player,
    None,
}

pub struct Score {
    player: usize,
    amy: usize,
}

impl Score {
    fn total(&self) -> usize {
        self.player + self.amy
    }
}

pub enum Rps {
    Rock,
    Paper,
    Scissor,
}

impl fmt::Display for Rps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rps::Rock => write!(f, "🪨"),
            Rps::Paper => write!(f, "📄"),
            Rps::Scissor => write!(f, "✂️"),
        }
    }
}

pub enum RpsResult {
    Win,
    Lose,
    Tie,
}

impl Rps {
    pub fn shoot(&self, other: &Rps) -> RpsResult {
        match &self {
            Rps::Rock => match &other {
                Rps::Rock => RpsResult::Tie,
                Rps::Paper => RpsResult::Lose,
                Rps::Scissor => RpsResult::Win,
            },
            Rps::Paper => match &other {
                Rps::Rock => RpsResult::Win,
                Rps::Paper => RpsResult::Tie,
                Rps::Scissor => RpsResult::Lose,
            },

            Rps::Scissor => match &other {
                Rps::Rock => RpsResult::Lose,
                Rps::Paper => RpsResult::Win,
                Rps::Scissor => RpsResult::Tie,
            },
        }
    }
}

impl RpsGame {
    pub fn new(best_of: usize) -> RpsGame {
        RpsGame {
            best_of,
            played: 0,
            score: Score { player: 0, amy: 0 },
        }
    }

    pub fn has_won(&self) -> RpsWinner {
        if self.score.amy > self.best_of / 2 {
            return RpsWinner::Amy;
        } else if self.score.player > self.best_of / 2 {
            return RpsWinner::Player;
        }
        RpsWinner::None
    }
}

impl Game for RpsGame {
    fn gameloop(&mut self) -> GameResult {
        while let RpsWinner::None = self.has_won() {
            println!();
            println!("{}", "Choose a number:".blue());
            println!("[1] Rock");
            println!("[2] Paper");
            println!("[3] Scissor");
            print!("> ");
            io::stdout().flush().unwrap();

            let player_choice = match user_number() {
                Ok(choice) if (1..=3).contains(&choice) => match choice {
                    1 => Rps::Rock,
                    2 => Rps::Paper,
                    3 => Rps::Scissor,
                    _ => unreachable!(),
                },
                _ => {
                    println!("{}", "Enter a number 1-3.".red());
                    continue;
                }
            };

            let mut rng = rand::thread_rng();
            let amy_choice = [Rps::Rock, Rps::Paper, Rps::Scissor]
                .iter()
                .choose(&mut rng)
                .unwrap();

            println!();
            println!("{}", "Rock, Paper, Scissors, Shoot!".yellow());
            println!();

            let mut player_marker = "  ";
            let mut amy_marker = "  ";
            match player_choice.shoot(amy_choice) {
                RpsResult::Win => {
                    self.score.player += 1;
                    player_marker = "->";
                }
                RpsResult::Tie => {}
                RpsResult::Lose => {
                    self.score.amy += 1;
                    amy_marker = "->";
                }
            }

            println!(
                "{}",
                format!("{} YOU chose {}.", player_marker, player_choice).white()
            );
            println!(
                "{}",
                format!("{} AMY chose {}.", amy_marker, amy_choice).white()
            );
            println!();

            self.played += 1;
            println!(
                "{}",
                format!("AMY: {} - YOU: {}", self.score.amy, self.score.player).magenta()
            );
        }

        match self.has_won() {
            RpsWinner::Amy => {
                println!("{}", "AMY Wins!".red());
                GameResult {
                    winner: Winner::Amy,
                    award: 1,
                }
            }
            RpsWinner::Player => {
                println!("{}", "You Win!".green());
                GameResult {
                    winner: Winner::Player,
                    award: 1,
                }
            }
            RpsWinner::None => unreachable!(),
        }
    }
}
