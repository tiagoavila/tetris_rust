use macroquad::prelude::*;

use crate::{
    core::{
        board::Board,
        constants::{BOARD_WIDTH, BOARD_X, BOARD_Y, BORDER_THICKNESS, CELL_SIZE},
        game::Game,
    },
    enums::{CellType, PieceType, RotationDirection},
    ui::render_engine,
};

mod core {
    pub mod board;
    pub mod constants;
    pub mod game;
    pub mod piece;
    pub mod point_2d;
}

mod ui {
    pub mod render_engine;
}

mod enums;

#[macroquad::main("Tetris Grid")]
async fn main() {
    let mut game = Game::new();
    let mut last_update = get_time();

    game.start();

    loop {
        clear_background(BLACK);

        // Handle user input
        if is_key_released(KeyCode::Left) || is_key_released(KeyCode::A) {
            game.move_piece_left();
        }

        if is_key_released(KeyCode::Right) || is_key_released(KeyCode::D) {
            game.move_piece_right();
        }

        if is_key_released(KeyCode::Up)
            || is_key_released(KeyCode::K)
            || is_key_released(KeyCode::W)
            || is_key_released(KeyCode::X)
        {
            game.rotate_piece(RotationDirection::Clockwise);
        }

        if is_key_released(KeyCode::J) || is_key_released(KeyCode::Q) || is_key_released(KeyCode::Z)
        {
            game.rotate_piece(RotationDirection::CounterClockwise);
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            macroquad::text::draw_text(
                "pressing s or down",
                BOARD_X + BOARD_WIDTH + 10.0,
                BOARD_Y + 20.0,
                20.0,
                WHITE,
            );
            game.start_soft_drop();
        }

        if is_key_released(KeyCode::S) || is_key_released(KeyCode::Down) {
            game.stop_soft_drop();
        }

        if is_key_released(KeyCode::Space) {
            game.hard_drop();
        }

        // Update piece position every interval
        let now = get_time();
        if now - last_update > game.fall_speed_seconds_per_line {
            game.do_on_each_loop();
            last_update = now;
        }

        render_engine::draw_board(&game.board);
        if let Some(ref mut game_piece) = game.current_piece {
            render_engine::draw_piece(&game_piece);
        }

        render_engine::draw_next_piece_section(&game.next_piece);

        next_frame().await;
    }
}
