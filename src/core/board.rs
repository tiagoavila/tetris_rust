use std::collections::HashMap;

use crate::{
    core::{
        constants::{COLS, ROWS},
        piece::Piece,
    },
    enums::cell_type::CellType,
};

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<CellType>>,
}

impl Board {
    pub fn new() -> Self {
        let filled_cells = vec![vec![CellType::Empty; COLS]; ROWS];
        Self {
            rows: ROWS,
            cols: COLS,
            cells: filled_cells,
        }
    }

    pub fn get_filled_cells(&self) -> HashMap<(usize, usize), CellType> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(row, cells)| {
                cells.iter().enumerate().filter_map(move |(col, cell)| {
                    if *cell != CellType::Empty {
                        Some(((row, col), *cell))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell_type: CellType) {
        if Piece::is_in_bounds(row, col) {
            self.cells[row][col] = cell_type;
        }
    }

    pub fn get_board_representation(&self) -> Vec<Vec<usize>> {
        let mut matrix = vec![vec![0 as usize; self.cols]; self.rows];
        self.cells.iter().enumerate().for_each(|(row, cells)| {
            cells.iter().enumerate().for_each(|(col, cell)| {
                if let CellType::Filled(_color) = cell {
                    matrix[row][col] = 1 as usize;
                }
            });
        });
        matrix
    }
    
    pub fn print_board(&self) {
        for row in &self.get_board_representation() {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

#[test]
fn test_board_initialization() {
    let board = Board::new();
    assert_eq!(board.rows, ROWS);
    assert_eq!(board.cols, COLS);
    assert_eq!(board.cells.len(), ROWS);
    for row in &board.cells {
        assert_eq!(row.len(), COLS);
        assert_eq!(row.iter().all(|&cell| cell == CellType::Empty), true);
    }
}

// #[test]
// fn test_set_cell() {
//     let mut board = Board::new();
//     board.set_cell(0, 0, CellType::Filled(BLUE));
//     assert_eq!(board.cells[0][0], CellType::Filled(BLUE));
// }
