use amy::{games::*, *};

const AMY_LOGO: &str = "  __ _ _ __ ___  _   _ \
                      \n / _` | '_ ` _ \\| | | |\
                      \n| (_| | | | | | | |_| |\
                      \n \\__,_|_| |_| |_|\\__, |\
                      \n                 |___/  ";

const AMY_VERSION: f32 = 1.1;

fn boot() {
    [
        "Updating AMY kernel",
        "Syncing remote packages",
        "Installing language recognition",
        "Initializing game states",
        "Booting AMY shell",
    ]
    .iter()
    .for_each(|msg| {
        print!("{:<50}", msg);
        loading_bar(&"‖".green().to_string(), 20, 70);
        println!("  ✓ DONE");
    });
}

fn main() {
    let player = Player::Human(Stats::new("Billy".to_string()));
    let amy = Player::Amy(Stats::new("AMY".to_string()));

    let players = Players::new(amy, player);

    boot();
    println!("{}v{:0.1}", AMY_LOGO, AMY_VERSION);
    scroll_print("See you later!", std::time::Duration::from_millis(30));

    println!();

    // Hangman
    // let mut game = hangman::Hangman::new(random_word());
    // game.gameloop();

    // // Guessing
    // let mut game = guess::GuessingGame::new(1, 100);
    // game.gameloop();

    // Poison
    // let mut game = poison::Poison::new();
    // game.gameloop();

    // Rock Paper Scissors
    // let mut game = rps::RpsGame::new(4);
    // game.gameloop();
}
