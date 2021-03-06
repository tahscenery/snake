use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

/// Gets the coordinate of a point on the game board.
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// Converts the coordinate of a point on the game board to an unsigned 32-bit integer type.
pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

/// Draws a block to the screen.
pub fn draw_block(colour: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        colour,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g
    );
}

/// Draws a rectangle of `Block`s.
pub fn draw_rect(
    colour: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        colour,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g
    );
}