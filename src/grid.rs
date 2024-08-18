use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use log::warn;

const GRID_SIZE: usize = 64;
const GRID_FILE: &str = "grid_state.bin";

#[derive(Clone, Serialize, Deserialize)]
pub struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            cells: vec![vec![false; GRID_SIZE]; GRID_SIZE],
        }
    }

    pub fn switch(&mut self, x: usize, y: usize) {
        if x < GRID_SIZE && y < GRID_SIZE {
            self.cells[y][x] = !self.cells[y][x];
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let mut file = File::create(GRID_FILE)?;
        let mut buffer = [0u8; GRID_SIZE * GRID_SIZE / 8];

        for (i, row) in self.cells.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let byte_index = (i * GRID_SIZE + j) / 8;
                let bit_index = (i * GRID_SIZE + j) % 8;
                if cell {
                    buffer[byte_index] |= 1 << bit_index;
                }
            }
        }

        file.write_all(&buffer)
    }

    pub fn load_from_file() -> std::io::Result<Self> {
        let mut file = File::open(GRID_FILE)?;
        let mut buffer = [0u8; GRID_SIZE * GRID_SIZE / 8];
        file.read_exact(&mut buffer)?;

        let mut grid = Grid::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let byte_index = (i * GRID_SIZE + j) / 8;
                let bit_index = (i * GRID_SIZE + j) % 8;
                grid.cells[i][j] = (buffer[byte_index] & (1 << bit_index)) != 0;
            }
        }

        Ok(grid)
    }

    pub fn load_or_create() -> Self {
        match Grid::load_from_file() {
            Ok(loaded_grid) => loaded_grid,
            Err(_) => {
                let new_grid = Grid::new();
                if let Err(e) = new_grid.save_to_file() {
                    warn!("Failed to save initial grid state: {}", e);
                }
                new_grid
            }
        }
    }
}
