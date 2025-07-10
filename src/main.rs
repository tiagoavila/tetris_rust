use macroquad::prelude::*;

// Board dimensions
const ROWS: usize = 20;
const COLS: usize = 10;
const CELL_SIZE: f32 = 27.0; // size of each cell in pixels
const BORDER_THICKNESS: f32 = 4.0;

// Calculate the board's pixel size
const BOARD_WIDTH: f32 = CELL_SIZE * COLS as f32;
const BOARD_HEIGHT: f32 = CELL_SIZE * ROWS as f32;

// Board top-left corner position
const BOARD_X: f32 = 50.0;
const BOARD_Y: f32 = 30.0;

#[macroquad::main("Tetris Grid")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_board();
        draw_pieces();

        next_frame().await
    }
}

fn draw_board() {
    // Draw board border
    draw_rectangle_lines(
        BOARD_X - BORDER_THICKNESS / 2.0,
        BOARD_Y - BORDER_THICKNESS / 2.0,
        BOARD_WIDTH + BORDER_THICKNESS,
        BOARD_HEIGHT + BORDER_THICKNESS,
        BORDER_THICKNESS,
        GRAY,
    );

    draw_rectangle(BOARD_X, BOARD_Y, BOARD_WIDTH, BOARD_HEIGHT, WHITE);

    // Draw horizontal lines
    for row in 1..ROWS {
        let y = BOARD_Y + row as f32 * CELL_SIZE;
        draw_line(BOARD_X, y, BOARD_X + BOARD_WIDTH, y, 1.0, GRAY);
    }

    // Draw vertical lines
    for col in 1..COLS {
        let x = BOARD_X + col as f32 * CELL_SIZE;
        draw_line(x, BOARD_Y, x, BOARD_Y + BOARD_HEIGHT, 1.0, GRAY);
    }
}

fn draw_pieces() {
    draw_rectangle(BOARD_X + CELL_SIZE * 1.0, BOARD_Y, CELL_SIZE, CELL_SIZE, BLUE);
    draw_rectangle(BOARD_X + CELL_SIZE * 2.0, BOARD_Y, CELL_SIZE, CELL_SIZE, BLUE);
    draw_rectangle(BOARD_X + CELL_SIZE * 3.0, BOARD_Y, CELL_SIZE, CELL_SIZE, BLUE);
    draw_rectangle(BOARD_X + CELL_SIZE * 2.0, BOARD_Y + CELL_SIZE * 1.0, CELL_SIZE, CELL_SIZE, BLUE);
}