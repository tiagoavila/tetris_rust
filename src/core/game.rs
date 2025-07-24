use crate::{core::{board::Board, constants::ROWS, piece::Piece}, enums::CellType};

pub struct Game {
    pub board: Board,
    pub current_piece: Option<Piece>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_piece: None,
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
    
    pub fn move_piece_down(&mut self) {
        if let Some(piece) = &mut self.current_piece {
            piece.move_down();
        }
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
    fn test_game_initialization() {
        let game = Game::new();
        assert_eq!(game.board.rows, ROWS);
        assert_eq!(game.board.cols, COLS);
        assert!(game.current_piece.is_none());
    }
    
    #[test]
    fn test_board_with_filled_cells() {
        let mut game = Game::new();
        initialize_test_board(&mut game);
        let expected_board = get_expected_board_representation_on_initialization();
        assert_eq!(game.board.get_board_representation(), expected_board);
    }
    
    #[test]
    fn test_detect_collision_with_bottom_row() {
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
    fn test_detect_collision_with_filled_cells() {
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
    fn test_detect_collision_with_bottom_row_z_piece() {
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
    fn test_detect_collision_with_filled_cells_z_piece() {
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