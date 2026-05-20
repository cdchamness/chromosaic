use std::{collections::HashSet, fs, path::PathBuf};

use crate::grid::Coord;
use anyhow::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub name: String,
    moves: Vec<Coord>,
}

impl Piece {
    pub fn new(name: &str, base_moves: &[Coord]) -> Piece {
        let mut piece = Piece {
            name: name.to_string(),
            moves: vec![],
        };
        for base_move in base_moves {
            piece.build_moves(base_move);
        }
        piece
    }

    pub fn build_piece_list() -> Result<Vec<Piece>> {
        let path = PathBuf::from("piecelist.txt");
        let content = fs::read_to_string(&path)?;

        let mut pieces = Vec::new();
        for line in content.lines() {
            let line = line
                .split_once('#')
                .map_or(line, |(before, _after)| before)
                .trim();
            if line.is_empty() {
                continue;
            }
            match line.split_once(':') {
                Some((name, base_moves_str)) => {
                    let mut base_moves = Vec::new();

                    for move_str in base_moves_str.split(';') {
                        if let Some((x_str, y_str)) = move_str.split_once(',') {
                            let x = x_str.trim().parse::<i32>()?;
                            let y = y_str.trim().parse::<i32>()?;
                            let base_move = Coord::new(x, y);
                            base_moves.push(base_move);
                        }
                    }
                    let new_piece = Piece::new(name, &base_moves);
                    pieces.push(new_piece);
                }
                None => continue,
            }
        }
        Ok(pieces)
    }

    pub fn from_name(name: &str, piece_list: &[Piece]) -> Piece {
        for piece in piece_list {
            if name == piece.name {
                return piece.clone();
            }
        }
        eprintln!("Could not find {} in piece_list", name);
        Piece::new("Knight", &[Coord::new(1, 2)])
    }

    fn build_moves(&mut self, base_move: &Coord) {
        let x = base_move.x;
        let y = base_move.y;
        let mut move_set = HashSet::new();
        move_set.insert(Coord::new(x, y));
        move_set.insert(Coord::new(x, -y));
        move_set.insert(Coord::new(-x, y));
        move_set.insert(Coord::new(-x, -y));
        move_set.insert(Coord::new(y, x));
        move_set.insert(Coord::new(y, -x));
        move_set.insert(Coord::new(-y, x));
        move_set.insert(Coord::new(-y, -x));

        let mut move_vec: Vec<Coord> = move_set.iter().cloned().collect();
        self.moves.append(&mut move_vec);
    }

    pub fn get_all_piece_types() -> Result<Vec<String>> {
        let piece_list = Piece::build_piece_list()?;
        let mut pieces: Vec<String> = piece_list.iter().map(|p| p.name.clone()).collect();
        pieces.sort();
        Ok(pieces)
    }

    pub fn moves(&self) -> &[Coord] {
        &self.moves
    }
}
