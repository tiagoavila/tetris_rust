// Board dimensions
pub(crate) const ROWS: usize = 20;
pub(crate) const COLS: usize = 10;
pub(crate) const CELL_SIZE: f32 = 27.0; // size of each cell in pixels
pub(crate) const BORDER_THICKNESS: f32 = 4.0;

// Calculate the board's pixel size
pub(crate) const BOARD_WIDTH: f32 = CELL_SIZE * COLS as f32;
pub(crate) const BOARD_HEIGHT: f32 = CELL_SIZE * ROWS as f32;

// Board top-left corner position
pub(crate) const BOARD_X: f32 = 50.0;
pub(crate) const BOARD_Y: f32 = 30.0;