use amy::{games::*, *};

const AMY_LOGO: &str = "  __ _ _ __ ___  _   _ \
                      \n / _` | '_ ` _ \\| | | |\
                      \n| (_| | | | | | | |_| |\
                      \n \\__,_|_| |_| |_|\\__, |\
                      \n                 |___/  ";

const AMY_VERSION: f32 = 1.0;

fn main() {
    println!("{}v{:0.1}", AMY_LOGO, AMY_VERSION);

    let player = Player::Human(Stats::new("Billy".to_string()));
    let amy = Player::Amy(Stats::new("AMY".to_string()));

    let players = Players::new(amy, player);

    // Hangman
    let mut game = hangman::Hangman::new(random_word());
    game.gameloop();

    // // Guessing
    let mut game = guess::GuessingGame::new(1, 100);
    game.gameloop();

    // Poison
    let mut game = poison::Poison::new();
    game.gameloop();

    // Rock Paper Scissors
    let mut game = rps::RpsGame::new(4);
    game.gameloop();
}
