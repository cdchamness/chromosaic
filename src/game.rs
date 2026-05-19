use std::collections::{HashMap, HashSet};

use eframe::egui::Color32;

use crate::{
    grid::{Coord, Spiral},
    piece::Piece,
};

pub struct Game {
    board: Spiral,
    players: Vec<Player>,
    coloring: HashMap<usize, usize>,
}

impl Game {
    pub fn new(board_size: usize, players: Vec<Player>) -> Game {
        Game {
            board: Spiral::new(board_size),
            players,
            coloring: HashMap::new(),
        }
    }

    pub fn play(&mut self) {
        if !self.coloring.is_empty() || self.players.is_empty() {
            return;
        }
        let player_count = self.players.len();

        let coord_to_index: HashMap<Coord, usize> = self
            .board
            .points()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, coord)| (coord, index))
            .collect();

        let mut occupied = HashSet::new();
        let mut attacked_by_player = vec![HashSet::new(); player_count];
        let mut next_candidate = vec![0_usize; player_count];

        let mut turn = 0;
        loop {
            let current_player = turn % player_count;
            let Some(index) =
                (next_candidate[current_player]..self.board.points().len()).find(|&index| {
                    !occupied.contains(&index)
                        && Game::not_attacked(&attacked_by_player, current_player, index)
                })
            else {
                break;
            };
            next_candidate[current_player] = index + 1;
            self.coloring.insert(index, current_player);
            occupied.insert(index);

            let current_coord = self.board.points()[index];
            for &offset in self.players[current_player].piece.moves() {
                let attacked_coord = current_coord + offset;
                if let Some(&attacked_index) = coord_to_index.get(&attacked_coord) {
                    attacked_by_player[current_player].insert(attacked_index);
                }
            }
            turn += 1;
        }
    }

    fn not_attacked(attacked: &[HashSet<usize>], current_color: usize, index: usize) -> bool {
        for (i, set) in attacked.iter().enumerate() {
            if i == current_color {
                continue;
            }
            if set.contains(&index) {
                return false;
            }
        }
        true
    }

    pub fn coloring(&self) -> &HashMap<usize, usize> {
        &self.coloring
    }

    pub fn game_name(&self) -> String {
        self.players
            .iter()
            .map(|p| p.piece.name.as_str())
            .collect::<Vec<_>>()
            .join("_")
    }

    pub fn board(&self) -> &Spiral {
        &self.board
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub piece: Piece,
    pub color: Color,
}

impl Player {
    pub fn new(piece: Piece, color: Color) -> Player {
        Player { piece, color }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn red() -> Color {
        Color { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Color {
        Color { r: 0, g: 255, b: 0 }
    }

    pub fn blue() -> Color {
        Color { r: 0, g: 0, b: 255 }
    }

    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }
}

impl Into<Color32> for Color {
    fn into(self) -> Color32 {
        Color32::from_rgb(self.r, self.g, self.b)
    }
}

impl From<Color32> for Color {
    fn from(value: Color32) -> Self {
        Color::from_rgb(value.r(), value.g(), value.b())
    }
}
