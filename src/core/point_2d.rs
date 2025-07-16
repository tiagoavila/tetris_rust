/// Represents a 2D point with integer coordinates. Y comes first, then X to match a matrix-like structure.
pub struct Point2D {
    pub y: isize,
    pub x: isize
}

impl Point2D {
    /// Creates a new [`Point2D`] with the specified coordinates.
    pub fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }

    /// Creates a default [`Point2D`] at the origin (0, 0).
    pub fn default() -> Self {
        Self { y: 0, x: 0 }
    }
}