use std::collections::{HashMap, HashSet};

use crate::{
    grid::{Coord, Spiral},
    piece::Piece,
};

pub struct Game {
    board: Spiral,
    colors: Vec<Color>,
    pieces: Vec<Piece>,
    coloring: HashMap<usize, usize>,
}

impl Game {
    pub fn new(board_size: usize, colors: Vec<Color>, pieces: Vec<Piece>) -> Game {
        Game {
            board: Spiral::new(board_size),
            colors,
            pieces,
            coloring: HashMap::new(),
        }
    }

    pub fn play(&mut self) {
        if !self.coloring.is_empty() {
            return;
        }
        let color_count = self.colors.len();

        let coord_to_index: HashMap<Coord, usize> = self
            .board
            .points()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, coord)| (coord, index))
            .collect();

        let mut occupied = HashSet::new();
        let mut attacked_by_color = vec![HashSet::new(); color_count];
        let mut next_candidate = vec![0_usize; color_count];

        let mut turn = 0;
        loop {
            let current_color = turn % color_count;
            let Some(index) =
                (next_candidate[current_color]..self.board.points().len()).find(|&index| {
                    !occupied.contains(&index)
                        && Game::not_attacked(&attacked_by_color, current_color, index)
                })
            else {
                break;
            };
            next_candidate[current_color] = index + 1;
            self.coloring.insert(index, current_color);
            occupied.insert(index);

            let current_coord = self.board.points()[index];
            for &offset in self.pieces[current_color].moves() {
                let attacked_coord = current_coord + offset;
                if let Some(&attacked_index) = coord_to_index.get(&attacked_coord) {
                    attacked_by_color[current_color].insert(attacked_index);
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
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
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
