use crate::{
    game::{Color, Game, Player},
    grid::{Coord, Spiral},
    piece::Piece,
    render::{image_dimensions, BoardBounds, ImageRenderOptions},
};

#[test]
fn builds_spiral_points() {
    let expected = [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(0, 1),
        Coord::new(-1, 1),
        Coord::new(-1, 0),
        Coord::new(-1, -1),
        Coord::new(0, -1),
        Coord::new(1, -1),
        Coord::new(2, -1),
        Coord::new(2, 0),
        Coord::new(2, 1),
        Coord::new(2, 2),
        Coord::new(1, 2),
        Coord::new(0, 2),
    ];

    assert_eq!(Spiral::new(expected.len()).points(), expected);
}

#[test]
fn colors_red_black_knights() -> anyhow::Result<()> {
    let piece_list = Piece::build_piece_list()?;
    let knight_piece = Piece::from_name("Knight", &piece_list);
    let knight_piece2 = Piece::from_name("Knight", &piece_list);

    let p1 = Player::new(knight_piece, Color::black());
    let p2 = Player::new(knight_piece2, Color::red());

    let mut game = Game::new(100, vec![p1, p2]);
    game.play();

    // These are listed on the ON-LINE ENCYCLOPEDIA OF INTEGER SEQUENCES
    let expected_black = [
        0, 2, 5, 9, 11, 15, 20, 21, 30, 31, 36, 40, 42, 47, 48, 50, 56, 61, 65, 67, 69, 70, 71, 75,
        76, 81, 83, 85, 87, 89, 93,
    ]; // OEIS A392177
    let expected_red = [
        1, 3, 4, 6, 10, 12, 24, 25, 34, 35, 37, 41, 44, 49, 55, 57, 58, 63, 64, 66, 68, 72, 78, 82,
        84, 86, 88, 90, 95, 96,
    ]; // OEIS A392178

    let mut black_actual: Vec<usize> = game
        .coloring()
        .iter()
        .filter_map(|(&square, &assigned_color)| (assigned_color == 0).then_some(square))
        .collect();
    black_actual.sort();

    let mut red_actual: Vec<usize> = game
        .coloring()
        .iter()
        .filter_map(|(&square, &assigned_color)| (assigned_color == 1).then_some(square))
        .collect();
    red_actual.sort();

    assert_eq!(black_actual, expected_black);
    assert_eq!(red_actual, expected_red);

    Ok(())
}

#[test]
fn bundled_piece_files_parse() -> anyhow::Result<()> {
    let piece_list = Piece::build_piece_list()?;
    for name in [
        "Alfil", "Camel", "Dabbaba", "Ferz", "Giraffe", "King", "Knight", "Wazir", "Zebra",
    ] {
        Piece::from_name(name, &piece_list);
    }

    Ok(())
}

#[test]
fn bundled_piece_types_are_found() -> anyhow::Result<()> {
    let types = Piece::get_all_piece_types()?;

    assert_eq!(
        types,
        ["Alfil", "Camel", "Dabbaba", "Ferz", "Giraffe", "King", "Knight", "Wazir", "Zebra",]
    );

    Ok(())
}

#[test]
fn board_bounds_cover_spiral_points() {
    let spiral = Spiral::new(10);
    let bounds = BoardBounds::from_points(spiral.points()).expect("spiral should not be empty");

    assert_eq!(bounds.min_x, -1);
    assert_eq!(bounds.max_x, 2);
    assert_eq!(bounds.min_y, -1);
    assert_eq!(bounds.max_y, 1);
    assert_eq!(bounds.width_cells(), 4);
    assert_eq!(bounds.height_cells(), 3);
}

#[test]
fn image_dimensions_include_padding() {
    let spiral = Spiral::new(10);
    let bounds = BoardBounds::from_points(spiral.points()).expect("spiral should not be empty");
    let options = ImageRenderOptions {
        cell_px: 8,
        padding_cells: 1,
    };

    assert_eq!(image_dimensions(bounds, options), (48, 40));
}
