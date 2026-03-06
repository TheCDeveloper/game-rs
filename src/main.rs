mod game;
mod sprite;
mod player;
use game::Game;

fn main() {
    let mut game = match Game::new() {
        Ok(g) => g,
        Err(e) => {
            panic!("Game crashed!\n{}", e);
        }
    };

    game.run();
}
