mod board;
mod game;
mod structures;

use game::Game;

pub fn run_game() -> Result<(), String> {
    let mut game = Game::with_size(1280, 720);
    game.game_loop()
}
