use macroquad::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Filled(Color),
}