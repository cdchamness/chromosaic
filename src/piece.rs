use std::{fs, path::PathBuf};

use crate::grid::Coord;
use anyhow::{Error, Result};

pub struct Piece {
    name: String,
    moves: Vec<Coord>,
}

impl Piece {
    pub fn from_name(name: &str) -> Result<Piece> {
        let mut path = PathBuf::from("pieces");
        path.push(format!("{name}.txt"));

        let content = fs::read_to_string(&path)?;

        let mut moves = Vec::new();
        for (line_index, line) in content.lines().enumerate() {
            let line = line.split_once('#').map_or(line, |(moves, _)| moves).trim();
            if line.is_empty() {
                continue;
            }

            let (dx, dy) = line.trim().split_once(',').ok_or_else(|| {
                Error::msg(format!(
                    "{name}.txt: line {} is invalid. Must look like dx,dy",
                    line_index + 1
                ))
            })?;
            let dx = dx.trim().parse::<i32>().map_err(|_| {
                Error::msg(format!(
                    "{name}.txt: line {} has an invalid 'dx'",
                    line_index + 1
                ))
            })?;
            let dy = dy.trim().parse::<i32>().map_err(|_| {
                Error::msg(format!(
                    "{name}.txt: line {} has an invalid 'dy'",
                    line_index + 1
                ))
            })?;
            if dx == 0 && dy == 0 {
                return Err(Error::msg(format!(
                    "{name}.txt: line {} cannout use the offset 0,0",
                    line_index + 1
                )));
            }

            moves.push(Coord::new(dx, dy))
        }
        if moves.is_empty() {
            return Err(Error::msg(format!(
                "{name}.txt does not contain any legal moves!"
            )));
        }
        let piece = Piece {
            name: name.to_string(),
            moves,
        };

        Ok(piece)
    }

    pub fn moves(&self) -> &[Coord] {
        &self.moves
    }
}
