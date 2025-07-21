use macroquad::color::Color;

use crate::{core::point_2d::Point2D, enums::{direction::Direction, piece_type::PieceType}};

pub struct Piece {
    pub piece_type: PieceType,
    pub blocks: Vec<Point2D>,
    pub position: Point2D,
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
    pub fn rotate(&mut self, direction: &Direction) {
        if self.piece_type == PieceType::O {
            // O piece does not rotate
            return;
        }

        match direction {
            Direction::Clockwise => self.rotate_cw(),
            Direction::CounterClockwise => self.rotate_ccw(),
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