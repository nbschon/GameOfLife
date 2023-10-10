use crate::game::Game;

mod board;
mod game;

fn main() {
    let mut game = Game::with_size(1280, 720);
    game.game_loop();
}
