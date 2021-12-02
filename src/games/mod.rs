pub mod guess;
pub mod hangman;
pub mod poison;

pub enum Player {
    Amy(Stats),
    Human(Stats),
}

impl Player {
    pub fn name(&self) -> &str {
        match self {
            Player::Amy(stats) => &stats.name,
            Player::Human(stats) => &stats.name,
        }
    }
}

pub struct Stats {
    name: String,
    points: i32,
}

impl Stats {
    pub fn new(name: String) -> Stats {
        Stats { name, points: 0 }
    }
}

pub struct Players {
    amy: Player,
    player: Player,
}

impl Players {
    pub fn new(amy: Player, player: Player) -> Players {
        Players { amy, player }
    }

    /// Returns a tuple containing (amy, player).
    pub fn get_players(&self) -> (&Player, &Player) {
        (&self.amy, &self.player)
    }
}

pub struct GameResult {
    winner: Winner,
    award: i32,
}

pub enum Winner {
    Amy,
    Human,
}

pub trait Game {
    fn gameloop(&mut self, players: &Players) -> GameResult;
}
