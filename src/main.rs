use macroquad::prelude::*;

use crate::{core::board::Board, enums::{cell_type::CellType, piece_type::PieceType}, ui::render_engine};

mod core{
    pub mod piece;
    pub mod point_2d;
    pub mod board;
    pub mod constants;
}

mod enums {
    pub mod direction;
    pub mod piece_type;
    pub mod cell_type;
}

mod ui {
    pub mod render_engine;
}

#[macroquad::main("Tetris Grid")]
async fn main() {
    let mut last_update = get_time();
    let update_interval = 0.5; // seconds
    let mut board = Board::new();
    board.set_cell(19, 5, CellType::Filled(BLUE));


    loop {
        clear_background(BLACK);

        // Update piece position every interval
        let now = get_time();
        if now - last_update > update_interval {
            // piece_y = (piece_y + 1) % ROWS;
            last_update = now;
        }

        render_engine::draw_board(&board);
        let row: isize = 3;
        let col: isize = 4;
        let piece = core::piece::Piece::new(&PieceType::T, core::point_2d::Point2D::new(row, col));
        render_engine::draw_piece(&piece);
        // draw_cell_piece(piece_y, 2);

        next_frame().await
    }
}
