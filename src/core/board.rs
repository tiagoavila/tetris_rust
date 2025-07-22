use std::collections::HashMap;

use crate::{core::constants::{COLS, ROWS}, enums::cell_type::CellType};

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    cells: Vec<Vec<CellType>>,
}

impl Board {
    pub fn new() -> Self {
        let filled_cells = vec![vec![CellType::Empty; COLS]; ROWS];
        Self { rows: ROWS, cols: COLS, cells: filled_cells }
    }

    pub fn get_filled_cells(&self) -> HashMap<(usize, usize), CellType> {
        self.cells.iter().enumerate().flat_map(|(row, cells)| {
            cells.iter().enumerate().filter_map(move |(col, cell)| {
                if *cell != CellType::Empty {
                    Some(((row, col), *cell))
                } else {
                    None
                }
            })
        }).collect()
    }
    
    pub fn set_cell(&mut self, row: usize, col: usize, cell_type: CellType) {
        if self.is_in_bounds(row, col) {
            self.cells[row][col] = cell_type;
        }
    }
    
    fn is_in_bounds(&self, row: usize, col: usize) -> bool {
        row > 0 && row < self.rows && col > 0 && col < self.cols
    }
}