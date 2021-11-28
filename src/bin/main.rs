use amy::{games::*, *};

const AMY_LOGO: &str = "  __ _ _ __ ___  _   _ \
                      \n / _` | '_ ` _ \\| | | |\
                      \n| (_| | | | | | | |_| |\
                      \n \\__,_|_| |_| |_|\\__, |\
                      \n                 |___/  ";

const AMY_VERSION: f32 = 1.0;

fn main() {
    println!("{}v{:0.1}", AMY_LOGO, AMY_VERSION);

    let player = Player::new("Billy".to_string());
    let amy = Player::new("Amy".to_string());

    // let mut game = hangman::Hangman::new(random_word());
    // let mut game = guess::GuessingGame::new(1, 100);
    let mut game = poison::Poison::new();
    game.gameloop((player, amy));
}
