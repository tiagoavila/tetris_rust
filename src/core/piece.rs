use macroquad::color::Color;

use crate::{core::{constants::{COLS, ROWS}, point_2d::Point2D}, enums::{direction::Direction, piece_type::PieceType, rotation_direction::RotationDirection}} ;

pub struct Piece {
    /// The type of the piece.
    pub piece_type: PieceType,

    /// A Vector of points relative to the piece's position with a centered origin represented as (0, 0).
    pub blocks: Vec<Point2D>,
    
    /// The position of the piece in the game area.
    /// The position is represented as a Point2D with the top-left corner as (0, 0).
    /// The x-coordinate represents the column and the y-coordinate represents the row.
    /// The position is used to determine where the piece is drawn on the board.
    pub position: Point2D,
    
    /// The color of the piece.
    /// This color is used to render the piece on the board.
    pub color: Color
}

impl Piece {
    /// Creates a new [`Piece`] based on the given `piece_type` and `position`.
    pub fn new(piece_type: &PieceType, position: Point2D) -> Self {
        Self {
            piece_type: piece_type.clone(),
            blocks: create_piece(piece_type),
            position,
            color: Self::get_color_by_piece(&piece_type),
        }
    }
    
    fn get_color_by_piece(piece_type: &PieceType) -> Color {
        match piece_type {
            PieceType::I => Color::new(0.0, 1.0, 1.0, 1.0), // Cyan
            PieceType::J => Color::new(0.0, 0.0, 1.0, 1.0), // Blue
            PieceType::L => Color::new(1.0, 0.5, 0.0, 1.0), // Orange
            PieceType::O => Color::new(1.0, 1.0, 0.0, 1.0), // Yellow
            PieceType::S => Color::new(0.0, 1.0, 0.5, 1.0), // Green
            PieceType::T => Color::new(1.0, 0.0, 1.0, 1.0), // Purple
            PieceType::Z => Color::new(1.0, 0.0, 0.5, 1.0), // Red
        }
    }
    
    /// Rotates the piece in the specified direction.
    /// Rotation applies a formula to the piece's blocks based on the direction.
    pub fn rotate(&mut self, direction: &RotationDirection) {
        if self.piece_type == PieceType::O {
            // O piece does not rotate
            return;
        }

        match direction {
            RotationDirection::Clockwise => self.rotate_cw(),
            RotationDirection::CounterClockwise => self.rotate_ccw(),
        }
    }
    
    /// Rotates the piece clockwise.
    /// This method modifies the piece's blocks in place.
    /// Clockwise rotation uses the formula:(y, x) -> (-x, y)
    fn rotate_cw(&mut self) {
        self.blocks = self.blocks.iter().map(|p| Point2D { y: -p.x, x: p.y }).collect::<Vec<Point2D>>();
    }
    
    /// Rotates the piece counter-clockwise.
    /// This method modifies the piece's blocks in place.
    /// Counter-clockwise rotation uses the formula:(y, x) -> (x, -y)
    fn rotate_ccw(&mut self) {
        self.blocks = self.blocks.iter().map(|p| Point2D { y: p.x, x: -p.y }).collect::<Vec<Point2D>>();
    }
    
    pub fn move_left(&mut self) {
        if self.can_move(&Direction::Left) {
            self.position.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.can_move(&Direction::Right) {
            self.position.x += 1;
        }
    }
    
    pub fn move_down(&mut self) {
        self.position.y += 1;
    }

    pub fn can_move(&self, direction: &Direction) -> bool {
        let new_col = match direction {
            Direction::Left => self.position.x - 1,
            Direction::Right => self.position.x + 1,
            _ => self.position.x
        };
        self.blocks.iter().all(|block| { 
            let block_col = new_col + block.x as isize;
            let block_row = self.position.y + block.y as isize;
            Piece::is_in_bounds(block_row as usize, block_col as usize) 
        })
    }

    /// Checks if the given row and column are within the bounds of the game area.
    pub fn is_in_bounds(row: usize, col: usize) -> bool {
        row >= 0 && row < ROWS && col >= 0 && col < COLS
    }
    
    pub fn get_blocks_position(&self) -> Vec<Point2D> {
        self.blocks.iter().map(|block| Point2D {
            x: self.position.x + block.x as isize,
            y: self.position.y + block.y as isize,
        }).collect()
    }
}

fn create_piece(piece_type: &PieceType) -> Vec<Point2D> {
    let blocks = match piece_type {
        PieceType::I => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(0, 2)],
        PieceType::J => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(-1, -1)],
        PieceType::L => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(-1, 1)],
        PieceType::O => vec![Point2D::new(0, 0), Point2D::new(-1, 0), Point2D::new(-1, 1), Point2D::new(0, 1)],
        PieceType::S => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(-1, 0), Point2D::new(-1, 1)],
        PieceType::T => vec![Point2D::new(-1, 0), Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1)],
        PieceType::Z => vec![Point2D::new(-1, -1), Point2D::new(-1, 0), Point2D::new(0, 0), Point2D::new(0, 1)],
    };
    blocks
}

#[test]
fn test_move_left_success() {
    let mut piece = Piece::new(&PieceType::I, Point2D::new(5, 5));
    let original_x = piece.position.x;
    piece.move_left();
    assert_eq!(piece.position.x, original_x - 1);
}

#[test]
fn test_move_right_success() {
    let mut piece = Piece::new(&PieceType::I, Point2D::new(5, 5));
    let original_x = piece.position.x;
    piece.move_right();
    assert_eq!(piece.position.x, original_x + 1);
}

#[test]
fn test_move_left_blocked() {
    // Place piece at left edge (x = 0)
    let mut piece = Piece::new(&PieceType::I, Point2D::new(5, 0));
    let original_x = piece.position.x;
    piece.move_left();
    // Should not move left
    assert_eq!(piece.position.x, original_x);
}

#[test]
fn test_move_right_blocked() {
    // Place piece at right edge (x = COLS - 1)
    let mut piece = Piece::new(&PieceType::I, Point2D::new(5, (COLS - 1) as isize));
    let original_x = piece.position.x;
    piece.move_right();
    // Should not move right
    assert_eq!(piece.position.x, original_x);
}