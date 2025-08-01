
use std::collections::HashSet;

use crate::{core::{board::Board, constants::{COLS, ROWS}, piece::Piece}, enums::{CellType, RotationDirection}};

pub struct Game {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub next_piece: Option<Piece>,
    pub fall_speed_seconds_per_line: f64,
    default_fall_speed: f64,
    fall_speed_soft_drop: f64, // Speed at which the piece falls
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_piece: None,
            next_piece: None,
            fall_speed_seconds_per_line: 1.0, // 1 Second per line
            default_fall_speed: 1.0,
            fall_speed_soft_drop: 1.0 / 20.0
        }
    }
    
    pub fn start(&mut self) {
        self.current_piece = Some(Piece::generate_random_piece());
        self.next_piece = Some(Piece::generate_random_piece());
    }

    pub fn move_piece_right(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.move_right();
        }
    }

    pub fn move_piece_left(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.move_left();
        }
    }
    
    pub fn move_piece_down(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.move_down();
        }
    }
    
    pub fn rotate_piece(&mut self, direction: RotationDirection) {
        if let Some(piece) = &mut self.current_piece {
            piece.rotate(direction);
        }
    }
    
    pub fn start_soft_drop(&mut self) {
        self.fall_speed_seconds_per_line = self.fall_speed_soft_drop;
    }
    
    pub fn stop_soft_drop(&mut self) {
        self.fall_speed_seconds_per_line = self.default_fall_speed;
    }
    
    pub fn hard_drop(&mut self) {
        loop {
            let collision = self.detect_collision();
            if collision {
                self.do_after_collision();
                break; // Stop when collision is detected
            } 

            if let Some(piece) = &mut self.current_piece {
                piece.move_down();
            }
        }
    }
    
    pub fn detect_collision(&self) -> bool {
        if let Some(piece) = &self.current_piece {
            for block in &piece.blocks {
                let row = piece.position.y + block.y as isize;
                if row == (ROWS - 1) as isize {
                    return true; // Collision with bottom
                }

                let col = piece.position.x + block.x as isize;
                // Check if cell bellow the current block is filled
                if self.board.cells[(row + 1) as usize][col as usize] != CellType::Empty {
                    return true; // Collision detected
                }
            }
        }

        false // No collision
    }
    
    pub fn detect_filled_rows(&mut self) {
        // let mut down_most_filled_row: Option<usize> = None;
        let mut filled_rows: HashSet<usize> = HashSet::new();

        self.board.cells = self.board.cells
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .filter_map(|(row_index, row)| {
                if row.iter().all(|cell| *cell != CellType::Empty) {
                    // Row is filled, replace with empty row
                    filled_rows.insert(row_index);
                    // if down_most_filled_row.is_none() || row_index > down_most_filled_row.unwrap() {
                    //     down_most_filled_row = Some(row_index);
                    // }
                    Some(vec![CellType::Empty; COLS])
                } else {
                    // Keep the row as is
                    Some(row)
                }
            })
            .rev()
            .collect();

        let mut filled_rows_vec: Vec<_> = filled_rows.iter().cloned().collect();
        filled_rows_vec.sort();
        for row_index in filled_rows_vec {
            // Shift all rows above the filled row down
            for r in (1..=row_index).rev() {
                if self.board.cells[r].iter().all(|cell| *cell != CellType::Empty) {
                    break; //stop loop when finding an empty row
                }

                self.board.cells[r] = self.board.cells[r - 1].clone();
            }
            // Set the top row to empty
            self.board.cells[0] = vec![CellType::Empty; COLS];
        }
    }
    
    pub fn do_on_each_loop(&mut self) {
        self.move_piece_down();
        //TODO: implement game over conditons
        if self.detect_collision() {
            self.do_after_collision();
        }
    }

    fn do_after_collision(&mut self) {
        self.board.place_piece(&self.current_piece.as_ref().unwrap().clone());
        self.detect_filled_rows();
        self.current_piece = self.next_piece.clone();
        self.next_piece = Some(Piece::generate_random_piece());
    }
    
    pub fn print_board_with_current_piece(&self) {
        let mut board_representation = self.board.get_board_representation();
        if let Some(piece) = &self.current_piece {
            let blocks_position = piece.get_blocks_position();
            for block in &blocks_position {
                let row = block.y as usize;
                let col = block.x as usize;
                board_representation[row][col] = 1; // Mark the piece's blocks
            }
        } 

        for row in board_representation.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use macroquad::color::BLUE;

    use crate::{core::{constants::COLS, point_2d::Point2D}, enums::PieceType};

    use super::*;

    #[test]
    fn game_initialization() {
        let game = Game::new();
        assert_eq!(game.board.rows, ROWS);
        assert_eq!(game.board.cols, COLS);
        assert!(game.current_piece.is_none());
    }
    
    #[test]
    fn board_with_filled_cells() {
        let mut game = Game::new();
        initialize_test_board(&mut game);
        let expected_board = get_expected_board_representation_on_initialization();
        assert_eq!(game.board.get_board_representation(), expected_board);
    }
    
    #[test]
    fn detect_collision_with_bottom_row() {
        let mut game = Game::new();
        initialize_test_board(&mut game);
        game.current_piece = Some(Piece::new(&PieceType::I, Point2D::new(18, 7)));
        assert_eq!(game.detect_collision(), false);
        // this Board will look like this:
        // row 0 to 17: 0 0 0 0 0 0 0 0 0 0
        // row 18:      1 0 0 0 0 0 1 1 1 1 // I piece is here at the right side
        // row 19:      1 0 0 0 1 0 0 0 0 0
        
        game.move_piece_down();
        assert!(game.detect_collision()); // Collision with bottom
        // After moving down, the piece should collide with the bottom row.
        // After moving down, the board will look like this:
        // row 0 to 17: 0 0 0 0 0 0 0 0 0 0 
        // row 18:      1 0 0 0 0 0 0 0 0 0 // I piece moved down
        // row 19:      1 0 0 0 1 0 1 1 1 1 // I piece is here at the bottom
    }
    
    #[test]
    fn detect_collision_with_filled_cells() {
        let mut game = Game::new();
        initialize_test_board(&mut game);
        game.current_piece = Some(Piece::new(&PieceType::I, Point2D::new(16, 1)));
        assert_eq!(game.detect_collision(), false);
        // this Board will look like this:
        // row 0 to 15: 0 0 0 0 0 0 0 0 0 0
        // row 16:      1 1 1 1 0 0 0 0 0 0 // I piece is here
        // row 17:      0 0 0 0 0 0 0 0 0 0
        // row 18:      1 0 0 0 0 0 0 0 0 0
        // row 19:      1 0 0 0 1 0 0 0 0 0
        
        game.move_piece_down();
        assert!(game.detect_collision()); // Collision with bottom
        // After moving down, the piece should collide with the filled cells below it.
        // After moving down, the board will look like this:
        // row 0 to 15: 0 0 0 0 0 0 0 0 0 0
        // row 16:      0 0 0 0 0 0 0 0 0 0 // I piece moved down
        // row 17:      1 1 1 1 0 0 0 0 0 0 // I piece is here
        // row 18:      1 0 0 0 0 0 0 0 0 0 // Collision if filled cell here
        // row 19:      1 0 0 0 1 0 0 0 0 0
    }

    #[test]
    fn detect_collision_with_bottom_row_z_piece() {
        let mut game = Game::new();
        initialize_test_board(&mut game);
        // Place Z piece just above the bottom row
        game.current_piece = Some(Piece::new(&PieceType::Z, Point2D::new(18, 2)));
        assert_eq!(game.detect_collision(), false);

        // this Board will look like this before moving down:
        // row 0 to 17: 0 1 1 0 0 0 0 0 0 0 // Z piece is here 
        // row 18:      1 0 1 1 0 0 0 0 0 0 // Z piece is here
        // row 19:      1 0 0 0 1 0 0 0 0 0 

        // Move Z piece down, should now collide with the bottom row
        game.move_piece_down();
        assert!(game.detect_collision());
        // game.print_board_with_current_piece();
        // After moving down, the board will look like this:
        // row 0 to 17: 0 0 0 0 0 0 0 0 0 0
        // row 18:      1 1 1 0 0 0 0 0 0 0 // Z piece moved down
        // row 19:      1 0 1 1 1 0 0 0 0 0 // Z piece is here 
        // This checks that the Z piece collides with the bottom row after moving down.
    }

    #[test]
    fn detect_collision_with_filled_cells_z_piece() {
        let mut game = Game::new();
        initialize_test_board(&mut game);
        // Place Z piece so that its lower block will collide after moving down
        game.current_piece = Some(Piece::new(&PieceType::Z, Point2D::new(17, 1)));
        assert_eq!(game.detect_collision(), false);
        game.board.print_board();

        // this Board will look like this before moving down:
        // row 0 to 16: 1 1 0 0 0 0 0 0 0 0 // Z piece is here
        // row 17:      0 1 1 0 0 0 0 0 0 0 // Z piece is here 
        // row 18:      1 0 0 0 0 0 0 0 0 0 
        // row 19:      1 0 0 0 1 0 0 0 0 0

        // Move Z piece down, should now collide with filled cell at (19, 0)
        game.move_piece_down();
        assert!(game.detect_collision());
        // After moving down, the board will look like this:
        // row 0 to 16: 0 0 0 0 0 0 0 0 0 0
        // row 17:      0 0 0 0 0 0 0 0 0 0 // Z piece moved down
        // row 18:      0 1 1 0 0 0 0 0 0 0 // Z piece is here
        // row 19:      1 1 0 0 1 0 0 0 0 0 // Z piece is here (collides with filled cell at (19,0))
        // This checks that the Z piece collides with the filled cell below after moving down.
    }
    
    #[test]
    fn detect_filled_row_should_not_change_when_there_are_no_filled_rows() {
        let mut game = Game::new();
        initialize_test_board(&mut game);
        let before = game.board.get_board_representation();
        game.print_board_with_current_piece();
        game.detect_filled_rows();
        println!("After detecting filled row:");
        let after = game.board.get_board_representation();
        game.print_board_with_current_piece();

        // The board should remain unchanged
        assert_eq!(before, after);
    }
    
    #[test]
    fn detect_filled_row() {
        let mut game = Game::new();
        // Fill the last row
        for col in 0..COLS {
            game.board.set_cell(19, col, CellType::Filled(BLUE));
        }
        // Fill some other cells for control
        game.board.set_cell(18, 0, CellType::Filled(BLUE));
        game.board.set_cell(18, 1, CellType::Filled(BLUE));

        let before = game.board.get_board_representation();
        game.print_board_with_current_piece();
        game.detect_filled_rows();
        println!("After detecting filled row:");
        let after = game.board.get_board_representation();
        game.print_board_with_current_piece();

        // The row where before it had some filled cells should now be empty
        assert_eq!(after[18], vec![0; COLS]);

        // The filled cell of the row above should be moved down
        assert_eq!(after[19][0], 1);
        assert_eq!(after[19][1], 1);

        // The rest of the board should be unchanged because they are all empty cells
        for row in 0..18 { // remember the for is exclusive of the last row
            assert_eq!(before[row], after[row]);
        }
    }
    
    #[test]
    fn detect_filled_row_when_there_are_multiple_filled_rows() {
        let mut game = Game::new();
        // Fill the last two rows
        for col in 0..COLS {
            game.board.set_cell(19, col, CellType::Filled(BLUE));
            game.board.set_cell(18, col, CellType::Filled(BLUE));
        }
        
        // Fill some other cells for control
        game.board.set_cell(17, 0, CellType::Filled(BLUE));
        game.board.set_cell(17, 1, CellType::Filled(BLUE));

        let before = game.board.get_board_representation();
        game.print_board_with_current_piece();
        game.detect_filled_rows();
        println!("After detecting filled row:");
        let after = game.board.get_board_representation();
        game.print_board_with_current_piece();

        // The last two rows should now be empty
        assert_eq!(after[17], vec![0; COLS]);
        assert_eq!(after[18], vec![0; COLS]);

        // Filled cells from rrow 17 should be moved down
        assert_eq!(after[19][0], 1);
        assert_eq!(after[19][1], 1);

        // The rest of the board should be unchanged because they are all empty cells, touched rows were 17, 18 and 19
        for row in 0..17 { // remember the for is exclusive of the last row
            assert_eq!(before[row], after[row]);
        }
    }
    
    #[test]
    fn detect_filled_row_when_there_are_multiple_filled_rows_and_some_empty_rows() {
        let mut game = Game::new();
        // Fill two rows
        for col in 0..COLS {
            game.board.set_cell(17, col, CellType::Filled(BLUE));
            game.board.set_cell(18, col, CellType::Filled(BLUE));
            game.board.set_cell(19, col, CellType::Filled(BLUE));
        }

        // set some cells to empty in the row in between
        game.board.set_cell(18, 9, CellType::Empty);
        game.board.set_cell(18, 8, CellType::Empty);
        
        // Fill some cells in row 16 for control
        game.board.set_cell(16, 0, CellType::Filled(BLUE));
        game.board.set_cell(16, 1, CellType::Filled(BLUE));
        game.board.set_cell(16, 7, CellType::Filled(BLUE));
        
        // Board representation before detecting filled rows
        // row 0 to 15: 0 0 0 0 0 0 0 0 0 0
        // row 16:      1 1 0 0 0 0 0 1 0 0
        // row 17:      1 1 1 1 1 1 1 1 1 1
        // row 18:      1 1 1 1 1 1 1 1 0 0
        // row 19:      1 1 1 1 1 1 1 1 1 1

        let before = game.board.get_board_representation();
        game.print_board_with_current_piece();
        game.detect_filled_rows();
        println!("After detecting filled row:");
        let after = game.board.get_board_representation();
        game.print_board_with_current_piece();

        // The top two rows should now be empty
        assert_eq!(after[16], vec![0; COLS]);
        assert_eq!(after[17], vec![0; COLS]);

        // Cells from row 18 should be moved down
        assert_eq!(after[19][9], 0);
        assert_eq!(after[19][8], 0);
        for col in 0..8 {
            assert_eq!(after[19][col], 1);
        }
        
        // Cells from row 16 should be moved down
        assert_eq!(after[18][0], 1);
        assert_eq!(after[18][1], 1);
        assert_eq!(after[18][7], 1);
        for col in 2..7 {
            assert_eq!(after[18][col], 0);
        }
        assert_eq!(after[18][8], 0);
        assert_eq!(after[18][9], 0);

        // The rest of the board should be unchanged except the filled rows that were removed
        for row in 0..16 { // remember the for is exclusive of the last row
            assert_eq!(before[row], after[row]);
        }
    }

    fn initialize_test_board(game: &mut Game) {
        game.board.set_cell(19, 0, CellType::Filled(BLUE));
        game.board.set_cell(19, 4, CellType::Filled(BLUE));

        game.board.set_cell(18, 0, CellType::Filled(BLUE));

        // this Board will look like this:
        // row 0 to 17: 0 0 0 0 0 0 0 0 0 0
        // row 18:      1 0 0 0 0 0 0 0 0 0
        // row 19:      1 0 0 0 1 0 0 0 0 0
    }
    
    fn get_expected_board_representation_on_initialization() -> Vec<Vec<usize>> {
        let expected_board: Vec<Vec<usize>> = vec![
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![0; COLS],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        ];

        expected_board
    }
}