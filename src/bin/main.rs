use amy::{games::*, *};

const AMY_LOGO: &str = "  __ _ _ __ ___  _   _ \
                      \n / _` | '_ ` _ \\| | | |\
                      \n| (_| | | | | | | |_| |\
                      \n \\__,_|_| |_| |_|\\__, |\
                      \n                 |___/  ";

const AMY_VERSION: f32 = 1.1;
const SCROLL_SPEED: u64 = 30;

// AMY boot up screen
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
    // boot();
    println!("{}v{:0.1}", AMY_LOGO, AMY_VERSION);

    println!();
    scroll_print(
        "Hello! My name is AMY. What is your name?",
        Duration::from_millis(SCROLL_SPEED),
    );

    print!("> ");
    io::stdout().flush().unwrap();

    let name = user_string();
    let players = Players::new(name);

    println!();
    scroll_print(
        &format!("Nice to meet you {}!", players.player()),
        Duration::from_millis(SCROLL_SPEED),
    );

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
