pub mod hangman;
pub mod guess;

pub struct Player {
    name: String,
    points: i32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player { name, points: 0 }
    }
}

pub struct GameResult {
    winner: Player,
    award: i32,
}

pub trait Game {
    fn gameloop(&mut self, players: (Player, Player)) -> GameResult;
}
