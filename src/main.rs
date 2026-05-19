use crate::{
    game::{Color, Game},
    piece::Piece,
};
use anyhow::Result;

mod game;
mod grid;
mod piece;

#[cfg(test)]
mod tests;

fn main() -> Result<()> {
    let knight_piece = Piece::from_name("knight")?;
    let knight_piece2 = Piece::from_name("knight")?;

    let red = Color::red();
    let blue = Color::blue();

    let mut game = Game::new(100, vec![red, blue], vec![knight_piece, knight_piece2]);
    game.play();

    Ok(())
}
