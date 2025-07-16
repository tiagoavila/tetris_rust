use crate::{core::point_2d::Point2D, enums::{direction::Direction, piece_type::PieceType}};

pub struct Piece {
    piece_type: PieceType,
    blocks: Vec<Point2D>,
    position: Point2D,
}

impl Piece {
    /// Creates a new [`Piece`] based on the given `piece_type` and `position`.
    pub fn new(piece_type: PieceType, position: Point2D) -> Self {
        let blocks = match piece_type {
            PieceType::I => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(0, 2)],
            PieceType::J => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(-1, -1)],
            PieceType::L => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(-1, 1)],
            PieceType::O => vec![Point2D::new(0, 0), Point2D::new(-1, 0), Point2D::new(-1, 1), Point2D::new(0, 1)],
            PieceType::S => vec![Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(-1, 0), Point2D::new(-1, 1)],
            PieceType::T => vec![Point2D::new(-1, 0), Point2D::new(0, -1), Point2D::new(0, 0), Point2D::new(0, 1)],
            PieceType::Z => vec![Point2D::new(-1, -1), Point2D::new(-1, 0), Point2D::new(0, 0), Point2D::new(0, 1)],
        };

        Self {
            piece_type,
            blocks,
            position,
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