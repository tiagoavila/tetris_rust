use macroquad::{color::{Color, GRAY, WHITE}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}};

use crate::{core::{board::Board, constants::*, piece::Piece}, enums::cell_type::CellType};


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
    board.get_filled_cells().iter().for_each(|((row, col), cell_type)| {
        if let CellType::Filled(color) = cell_type {
            draw_cell_piece(*row, *col, *color);
        }
    });
}

pub fn draw_piece(piece: &Piece) {
    for block in &piece.blocks {
        let row = (piece.position.y + block.y) as usize;
        let col = (piece.position.x + block.x) as usize;
        draw_cell_piece(row, col, piece.color);
    }
}

fn draw_cell_piece(row: usize, col: usize, color: Color) {
    let x = BOARD_X + CELL_SIZE * col as f32;
    let y = BOARD_Y + CELL_SIZE * row as f32;
    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
}