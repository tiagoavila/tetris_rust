use macroquad::{
    color::{BLUE, Color, GRAY, WHITE},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
};

use crate::{
    core::{board::Board, constants::*, piece::Piece},
    enums::cell_type::CellType,
};

pub fn draw_board(board: &Board) {
    draw_empty_board();
    draw_filled_cells(board);
}

fn draw_empty_board() {
    // Draw board border
    draw_rectangle_lines(
        BOARD_X - BORDER_THICKNESS / 2.0,
        BOARD_Y - BORDER_THICKNESS / 2.0,
        BOARD_WIDTH + BORDER_THICKNESS,
        BOARD_HEIGHT + BORDER_THICKNESS,
        BORDER_THICKNESS,
        GRAY,
    );

    // Draw the background of the board
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

fn draw_filled_cells(board: &Board) {
    // Draw filled cells
    board
        .get_filled_cells()
        .iter()
        .for_each(|((row, col), cell_type)| {
            if let CellType::Filled(color) = cell_type {
                draw_cell_piece(*row, *col, *color);
            }
        });
}

pub fn draw_piece(game_piece: &Option<Piece>) {
    if let Some(piece) = game_piece {
        for block in &piece.blocks {
            let row = (piece.position.y + block.y) as usize;
            let col = (piece.position.x + block.x) as usize;
            draw_cell_piece(row, col, piece.color);
        }
    }
}

pub fn draw_next_piece_section(next_piece: &Option<Piece>) {
    let x_position_text_placement = BOARD_X + BOARD_WIDTH + 40.0;
    let y_position_text_placement = BOARD_Y + 20.0;
    let y_position_next_piece_container = BOARD_Y + 40.0;
    macroquad::text::draw_text(
        "Next Piece",
        x_position_text_placement,
        y_position_text_placement,
        24.0,
        WHITE,
    );

    macroquad::shapes::draw_rectangle_lines(
        x_position_text_placement,
        y_position_next_piece_container,
        6.0 * CELL_SIZE + BORDER_THICKNESS,
        4.0 * CELL_SIZE + BORDER_THICKNESS,
        BORDER_THICKNESS,
        BLUE,
    );

    macroquad::shapes::draw_rectangle(
        x_position_text_placement + BORDER_THICKNESS / 2.0,
        y_position_next_piece_container + BORDER_THICKNESS / 2.0,
        6.0 * CELL_SIZE,
        4.0 * CELL_SIZE,
        WHITE,
    );

    if let Some(next_piece) = next_piece {
        draw_next_piece(next_piece);
    }
}

pub fn draw_level_and_speed(level: usize, drop_speed: f64) {
    let x_position = BOARD_X + BOARD_WIDTH + 40.0;
    let y_position = BOARD_Y + 40.0 + 4.0 * CELL_SIZE + 40.0;
    let level_text = format!("Level: {}", level);
    let speed_text = format!("Drop Speed: {:.2}s", drop_speed);

    macroquad::text::draw_text(&level_text, x_position, y_position, 24.0, BLUE);
    macroquad::text::draw_text(&speed_text, x_position, y_position + 30.0, 24.0, BLUE);
}

pub fn draw_controls() {
    let x_position = BOARD_X + BOARD_WIDTH + 40.0;
    let y_position = BOARD_Y + 40.0 + 4.0 * CELL_SIZE + 110.0;
    let controls = [
        "Controls:",
        "Left Arrow / A: Move Left",
        "Right Arrow / D: Move Right",
        "Up Arrow / W / X: Rotate Clockwise",
        "Q / Z: Rotate Counter-Clockwise",
        "Down Arrow / S: Soft Drop",
        "Space: Hard Drop",
        "Esc: Quit/Close"
    ];
    for (i, text) in controls.iter().enumerate() {
        macroquad::text::draw_text(text, x_position, y_position + i as f32 * 22.0, 20.0, WHITE);
    }
}

pub fn draw_game_over_message() {
    let screen_width = macroquad::window::screen_width();
    let screen_height = macroquad::window::screen_height();

    // Rectangle dimensions
    let rect_height = 120.0; // Increased height for additional text
    let rect_y = (screen_height - rect_height) / 2.0;
    let border_thickness = 3.0;

    // Draw main rectangle (background)
    macroquad::shapes::draw_rectangle(
        0.0,
        rect_y,
        screen_width,
        rect_height,
        macroquad::color::DARKGRAY,
    );

    // Draw top border
    macroquad::shapes::draw_rectangle(
        0.0,
        rect_y,
        screen_width,
        border_thickness,
        macroquad::color::WHITE,
    );

    // Draw bottom border
    macroquad::shapes::draw_rectangle(
        0.0,
        rect_y + rect_height - border_thickness,
        screen_width,
        border_thickness,
        macroquad::color::WHITE,
    );

    // Main "Game Over" text
    let main_text = "Game Over";
    let main_font_size = 40.0;
    let main_dimensions =
        macroquad::text::measure_text(main_text, None, main_font_size as u16, 1.0);
    let main_text_x = (screen_width - main_dimensions.width) / 2.0;
    let main_text_y = rect_y + 40.0; // Position towards top of rectangle

    macroquad::text::draw_text(
        main_text,
        main_text_x,
        main_text_y,
        main_font_size,
        macroquad::color::RED,
    );

    // Instructions text
    let instruction_text = "Press ESC or SPACE to close";
    let instruction_font_size = 24.0;
    let instruction_dimensions =
        macroquad::text::measure_text(instruction_text, None, instruction_font_size as u16, 1.0);
    let instruction_text_x = (screen_width - instruction_dimensions.width) / 2.0;
    let instruction_text_y = rect_y + 80.0; // Below the main text

    macroquad::text::draw_text(
        instruction_text,
        instruction_text_x,
        instruction_text_y,
        instruction_font_size,
        macroquad::color::WHITE,
    );
}

fn draw_next_piece(piece: &Piece) {
    let x_position_text_placement = BOARD_X + BOARD_WIDTH - 12.0;
    let y_position_next_piece_container = BOARD_Y + 40.0 + CELL_SIZE;

    for block in &piece.blocks {
        let row = (piece.position.y + block.y) as usize;
        let col = (piece.position.x + block.x) as usize;
        let x = x_position_text_placement + CELL_SIZE * col as f32;
        let y = y_position_next_piece_container + CELL_SIZE * row as f32;
        draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, piece.color);
    }
}

fn draw_cell_piece(row: usize, col: usize, color: Color) {
    let x = BOARD_X + CELL_SIZE * col as f32;
    let y = BOARD_Y + CELL_SIZE * row as f32;
    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
}
