use crate::{games::*, *};
use std::collections::HashMap;

pub struct Poison {
    current_number: usize,
    target_number: usize,
    map: HashMap<usize, Player>,
}

impl Poison {
    pub fn new() -> Poison {
        Poison {
            current_number: 0,
            target_number: 21,
            map: HashMap::new(),
        }
    }
}

pub enum PoisonTurn {
    Amy,
    Player,
}

impl PoisonTurn {
    fn flip(&mut self) {
        *self = match self {
            PoisonTurn::Amy => PoisonTurn::Player,
            PoisonTurn::Player => PoisonTurn::Amy,
        }
    }
}

impl Game for Poison {
    fn gameloop(&mut self, players: (Player, Player)) -> GameResult {
        let mut turn = match coin_flip() {
            Coin::Heads => PoisonTurn::Amy,
            Coin::Tails => PoisonTurn::Player,
        };
        while self.current_number < self.target_number {
            match turn {
                PoisonTurn::Amy => println!("AMY's Turn"),
                PoisonTurn::Player => println!("Player's Turn"),
            };
            turn.flip();
        }
        todo!()
    }
}
