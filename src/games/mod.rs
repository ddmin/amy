pub mod guess;
pub mod hangman;
pub mod poison;
pub mod rps;

struct PlayerInfo {
    name: String,
    points: i32,
}

impl PlayerInfo {
    fn new(name: String) -> PlayerInfo {
        PlayerInfo { name, points: 0 }
    }
}

pub struct Players {
    amy: PlayerInfo,
    player: PlayerInfo,
}

impl Players {
    pub fn new(player_name: String) -> Players {
        Players {
            amy: PlayerInfo::new("AMY".to_string()),
            player: PlayerInfo::new(player_name),
        }
    }

    pub fn amy(&self) -> &str {
        &self.amy.name
    }

    pub fn player(&self) -> &str {
        &self.player.name
    }
}

pub struct GameResult {
    winner: Winner,
    award: i32,
}

pub enum Winner {
    Amy,
    Player,
}

pub trait Game {
    fn gameloop(&mut self) -> GameResult;
}
