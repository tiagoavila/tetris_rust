use macroquad::prelude::*;

use crate::{core::{board::Board, constants::ROWS}, enums::{cell_type::CellType, piece_type::PieceType}, ui::render_engine};

mod core{
    pub mod piece;
    pub mod point_2d;
    pub mod board;
    pub mod constants;
}

mod enums {
    pub mod cell_type;
    pub mod direction;
    pub mod piece_type;
    pub mod rotation_direction;
}

mod ui {
    pub mod render_engine;
}

#[macroquad::main("Tetris Grid")]
async fn main() {
    let mut last_update = get_time();
    let update_interval = 1.2; // seconds
    let mut row: isize = 3;
    let mut board = Board::new();
    board.set_cell(19, 5, CellType::Filled(BLUE));


    let mut piece = core::piece::Piece::new(&PieceType::S, core::point_2d::Point2D::new(row, 4));

    loop {
        clear_background(BLACK);

        // Handle user input
        if is_key_released(KeyCode::Left) {
            piece.move_left();
        }

        if is_key_released(KeyCode::Right) {
            piece.move_right();
        }

        // Update piece position every interval
        let now = get_time();
        if now - last_update > update_interval {
            row = (row + 1) % 20;
            last_update = now;
        }

        render_engine::draw_board(&board);
        render_engine::draw_piece(&piece);

        next_frame().await;
    }
}
