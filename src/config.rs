use anyhow::{Result, bail};

use crate::game::{Game, Player};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub board_size: usize,
    pub players: Vec<Player>,
}

impl AppConfig {
    pub fn new(board_size: usize, players: Vec<Player>) -> AppConfig {
        AppConfig {
            board_size,
            players,
        }
    }

    pub fn build_game(&self) -> Result<Game> {
        if self.board_size == 0 {
            bail!("board size must be at least 1");
        }
        if self.players.is_empty() {
            bail!("at least one player row is required");
        }

        let mut game = Game::new(self.board_size, self.players.clone());
        game.play();
        Ok(game)
    }
}
