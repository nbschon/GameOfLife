mod game_of_life;
use game_of_life::run_game;

pub fn run() -> Result<(), String> {
    Ok(run_game()?)
}
