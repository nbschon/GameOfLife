mod game;
mod board;
mod structures;

use game::Game;

pub fn run_game() -> Result<(), String> {
    let mut game = Game::with_size(1280, 720);
    game.game_loop()?;
    Ok(())
}
