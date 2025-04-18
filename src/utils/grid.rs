use crate::utils::maths::SirParams;
use rand::Rng;
use std::mem::size_of;

/// Two-bit encoding for three health states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HealthState {
    Susceptible = 0,
    Infected    = 1,
    Recovered   = 2,
}

/// Flat, bit-packed grid: 2 bits per cell, 4 cells per byte.
pub struct Grid {
    pub grid_x: usize,
    pub grid_y: usize,
    pub cells: Vec<u8>,  // 2 bits per cell packed into bytes
}
impl Grid {
    /// Initialize a new grid, randomly infecting according to params.i_ratio.
    pub fn init(grid_x: usize, grid_y: usize, params: &SirParams) -> Self {
        const MAX_CELLS: usize = 1_000_000_000;
        let size = grid_x.checked_mul(grid_y)
            .expect("Grid dimensions overflowed");

        if size > MAX_CELLS {
            panic!(
                "Grid too large: {}x{} = {} cells. Limit is {}.",
                grid_x, grid_y, size, MAX_CELLS
            );
        }
        // 4 cells per byte
        let byte_len = (size + 3) / 4;
        let mut cells = vec![0u8; byte_len];
        let mut rng = rand::thread_rng();
        for idx in 0..size {
            let roll: f64 = rng.r#gen();
            let state = if roll < params.i_ratio {
                HealthState::Infected
            } else {
                HealthState::Susceptible
            };
            Self::write_state(&mut cells, idx, state);
        }
        Grid { grid_x, grid_y, cells }
    }

    /// Internal helper: write directly to raw cell buffer
    fn write_state(cells: &mut [u8], idx: usize, state: HealthState) {
        let byte = idx / 4;
        let shift = (idx % 4) * 2;
        let mask = !(0b11 << shift);
        cells[byte] = (cells[byte] & mask) | ((state as u8) << shift);
    }

    /// Get cell index (linear), panics if out of bounds.
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.grid_x + x
    }

    /// Return the 8 neighbors' coordinates (still allocates Vec here).
    pub fn get_neighbors(&self, x: usize, y: usize, buffer: &mut [(usize, usize)]) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.grid_x as isize && ny >= 0 && ny < self.grid_y as isize {
                    buffer[count] = (nx as usize, ny as usize);
                    count += 1
                }
            }
        }
        count
    }

    /// Read the state at linear index.
    pub fn read(&self, idx: usize) -> HealthState {
        let byte = idx / 4;
        let shift = (idx % 4) * 2;
        match (self.cells[byte] >> shift) & 0b11 {
            0 => HealthState::Susceptible,
            1 => HealthState::Infected,
            2 => HealthState::Recovered,
            _ => unreachable!("Invalid state bits"),
        }
    }

    /// Write a state at linear index.
    pub fn write(&mut self, idx: usize, state: HealthState) {
        let byte = idx / 4;
        let shift = (idx % 4) * 2;
        let mask = !(0b11 << shift);
        self.cells[byte] = (self.cells[byte] & mask) | ((state as u8) << shift);
    }

    /// Prints approximate memory usage: 2 bits/cell packed in `cells.len()` bytes.
    pub fn get_grid_size(&self) -> (usize, usize, usize) {
        let bits_per_cell = 2;
        let heap_bytes = self.cells.len();
        let struct_bytes = std::mem::size_of::<Self>();
        println!("Bits per cell: {}", bits_per_cell);
        println!("Total heap usage: {} bytes (~{:.2} MB)", heap_bytes, heap_bytes as f64 / (1024.0*1024.0));
        println!("Grid struct size: {} bytes", struct_bytes);
        (bits_per_cell, heap_bytes, struct_bytes)
    }
}

pub struct Tile<'a> {
    pub origin_x: usize,
    pub origin_y: usize,
    pub tile_x: usize,
    pub tile_y: usize,
    pub grid: &'a Grid,
}

impl<'a> Tile<'a> {
    pub fn get_state(&self, x: usize, y: usize) -> Option<HealthState> {
        //Calculates the equiv cell in Grid
        let global_x = self.origin_x + x;
        let global_y = self.origin_y + y;

        //If equiv both cell is greater or equal retrun nothing 
        if global_x >= self.grid.grid_x || global_y >= self.grid.grid_y {
            return None;
        }

        //Get the linear index into the full grid for the given global coordinates (x, y)
        let idx = self.grid.get_index(global_x, global_y);
        
        //Read the HealthState at that index from the grid and return it
        //Some checks if the value is not null
        Some(self.grid.read(idx))
    }

    pub fn get_neighbors_healthstates(&self, x: usize, y: usize, buffer: &mut [Option<HealthState>; 8]) -> usize {
        let mut count = 0;
        // Loop over the 3x3 grid centered at (x, y)
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                 // Compute neighbor coordinates (may be negative)
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                // Try to convert isize -> usize safely (only works if >= 0)
                if let (Some(nx), Some(ny)) =
                    (nx.try_into().ok(), ny.try_into().ok())
                {
                    // Use get_state to retrieve the neighbor’s health state (returns Option)
                    if let Some(state) = self.get_state(nx, ny) {
                        // If the neighbor exists and is within bounds, store its state
                        buffer[count] = Some(state);
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

pub fn tile_grid(grid: &Grid, tile_width: usize, tile_height: usize) -> Vec<Tile> {
    let mut tiles = Vec::new();

    let num_tiles_x = (grid.grid_x + tile_width - 1) / tile_width;
    let num_tiles_y = (grid.grid_y + tile_height - 1) / tile_height;

    for tile_y in 0..num_tiles_y {
        for tile_x in 0..num_tiles_x {
            let origin_x = tile_x * tile_width;
            let origin_y = tile_y * tile_height;

            // Clamp width/height to avoid going out of bounds
            let actual_width = (origin_x + tile_width).min(grid.grid_x) - origin_x;
            let actual_height = (origin_y + tile_height).min(grid.grid_y) - origin_y;

            tiles.push(Tile {
                origin_x,
                origin_y,
                tile_x: actual_width,
                tile_y: actual_height,
                grid,
            });
        }
    }

    tiles
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::maths::SirParams;

    fn dummy_params(i_ratio: f64) -> SirParams {
        SirParams {
            beta: 0.0,
            gamma: 0.0,
            dt: 1.0,
            i_ratio,
            s_ratio: 1.0, // Fully susceptible for now
        }
    }

    #[test]
    fn test_gridinit_case1() {
        let params = dummy_params(0.0);
        let grid = Grid::init(10, 5, &params);
        assert_eq!(grid.grid_x, 10);
        assert_eq!(grid.grid_y, 5);
        assert_eq!(grid.cells.len(), (10 * 5 + 3) / 4); // expect 13 bytes
    }

    #[test]
    fn test_grid_get_grid_size_case1() {
        let params = dummy_params(0.0);
        let grid = Grid::init(100, 100, &params);
        let (bits_per_cell, heap_size, struct_size) = grid.get_grid_size();
    
        assert_eq!(bits_per_cell, 2);
        assert_eq!(heap_size, 2500); // 10000 cells / 4 = 2500 bytes
        assert!(struct_size > 0); // or check against actual value
    }

    #[test]
    fn test_grid_get_index_case1() {
        let params = dummy_params(0.0);
        let grid = Grid::init(10, 5, &params);
        assert_eq!(grid.get_index(3, 2), 23);
        assert_eq!(grid.get_index(0, 0), 0);
        assert_eq!(grid.get_index(9, 4), 49);
    }

    #[test]
    fn test_grid_get_neighbors_case1() {
        let params = dummy_params(0.0);
        let grid = Grid::init(20, 20, &params);
        let neighbors = grid.get_neighbors(10, 10);
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&(9, 9)));
        assert!(neighbors.contains(&(10, 9)));
        assert!(neighbors.contains(&(11, 11)));
    }

    #[test]
    fn test_grid_get_neighbors_case2() {
        let params = dummy_params(0.0);
        let grid = Grid::init(20, 20, &params);
        let neighbors = grid.get_neighbors(0, 0);
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(1, 1)));
    }

    #[test]
    fn test_grid_get_neighbors_case3() {
        let params = dummy_params(0.0);
        let grid = Grid::init(20, 20, &params);
        let neighbors = grid.get_neighbors(0, 10);
        assert_eq!(neighbors.len(), 5);
        assert!(neighbors.contains(&(0, 9)));
        assert!(neighbors.contains(&(1, 9)));
        assert!(neighbors.contains(&(1, 10)));
        assert!(neighbors.contains(&(0, 11)));
        assert!(neighbors.contains(&(1, 11)));
    }

    #[test]
    fn test_grid_tile_grid_case1() {
        use crate::utils::maths::SirParams;

        // Dummy params for grid initialization
        let params = SirParams {
            beta: 0.0,
            gamma: 0.0,
            dt: 1.0,
            i_ratio: 0.0,
            s_ratio: 1.0,
        };

        // 100x100 grid
        let grid = Grid::init(100, 100, &params);

        // Tile into 25x25 chunks
        let tiles = tile_grid(&grid, 25, 25);

        // Should be 4 x 4 = 16 tiles
        assert_eq!(tiles.len(), 16);

        // Check some key tile origins
        assert_eq!(tiles[0].origin_x, 0);
        assert_eq!(tiles[0].origin_y, 0);

        assert_eq!(tiles[1].origin_x, 25);
        assert_eq!(tiles[1].origin_y, 0);

        assert_eq!(tiles[4].origin_x, 0);
        assert_eq!(tiles[4].origin_y, 25);
        
    }
}
