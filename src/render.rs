use std::path::Path;

use anyhow::{Result, bail};
use image::{Rgba, RgbaImage};

use crate::{game::Game, grid::Coord};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardBounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl BoardBounds {
    pub fn from_points(points: &[Coord]) -> Option<BoardBounds> {
        let first = points.first()?;
        let mut bounds = BoardBounds {
            min_x: first.x,
            max_x: first.x,
            min_y: first.y,
            max_y: first.y,
        };

        for point in &points[1..] {
            bounds.min_x = bounds.min_x.min(point.x);
            bounds.max_x = bounds.max_x.max(point.x);
            bounds.min_y = bounds.min_y.min(point.y);
            bounds.max_y = bounds.max_y.max(point.y);
        }

        Some(bounds)
    }

    pub fn width_cells(&self) -> u32 {
        (self.max_x - self.min_x + 1) as u32
    }

    pub fn height_cells(&self) -> u32 {
        (self.max_y - self.min_y + 1) as u32
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageRenderOptions {
    pub cell_px: u32,
    pub padding_cells: u32,
}

impl Default for ImageRenderOptions {
    fn default() -> Self {
        ImageRenderOptions {
            cell_px: 16,
            padding_cells: 1,
        }
    }
}

pub fn image_dimensions(bounds: BoardBounds, options: ImageRenderOptions) -> (u32, u32) {
    let padding = options.padding_cells * 2;
    (
        (bounds.width_cells() + padding) * options.cell_px,
        (bounds.height_cells() + padding) * options.cell_px,
    )
}

pub fn write_png(path: impl AsRef<Path>, game: &Game, options: ImageRenderOptions) -> Result<()> {
    if options.cell_px == 0 {
        bail!("cell size must be at least one pixel");
    }

    let points = game.board().points();
    let Some(bounds) = BoardBounds::from_points(points) else {
        bail!("cannot export an empty board");
    };

    let (width, height) = image_dimensions(bounds, options);
    let background = Rgba([242, 242, 238, 255]);
    let uncolored = Rgba([220, 220, 216, 255]);
    let mut image = RgbaImage::from_pixel(width, height, background);

    for (index, coord) in points.iter().enumerate() {
        let color = game
            .coloring()
            .get(&index)
            .and_then(|color_index| game.players().get(*color_index))
            .map(|p| Rgba([p.color.r, p.color.g, p.color.b, 255]))
            .unwrap_or(uncolored);

        let x = (coord.x - bounds.min_x) as u32 + options.padding_cells;
        let y = (bounds.max_y - coord.y) as u32 + options.padding_cells;
        fill_cell(&mut image, x, y, options.cell_px, color);
    }

    image.save(path)?;
    Ok(())
}

fn fill_cell(image: &mut RgbaImage, cell_x: u32, cell_y: u32, cell_px: u32, color: Rgba<u8>) {
    let start_x = cell_x * cell_px;
    let start_y = cell_y * cell_px;
    for y in start_y..start_y + cell_px {
        for x in start_x..start_x + cell_px {
            image.put_pixel(x, y, color);
        }
    }
}
