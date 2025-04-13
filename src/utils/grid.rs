use crate::utils::maths::SirParams;
use rand::Rng;
use std::mem::size_of;

// ffsdg
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthState {
    Susceptible,
    Infected,
    Recovered,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub state: HealthState,
}

pub struct Grid {
    pub grid_x: usize,
    pub grid_y: usize,
    pub cells: Vec<Cell>, // Flattened 2D grid
}

impl Grid {
    /// Creates a new grid of given dimensions.
    /// Uses `i_ratio` from `SirParams` to determine the fraction of cells initialized as infected.
    pub fn init(grid_x: usize, grid_y: usize, params: &SirParams) -> Self {
        let size = grid_x * grid_y;

        let cells = (0..size)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let roll = rng.r#gen::<f64>(); // 🔥 This must use `gen` from `Rng`
                let state = if roll < params.i_ratio {
                    HealthState::Infected
                } else {
                    HealthState::Susceptible
                };
                Cell { state }
            })
            .collect();

        Self {
            grid_x,
            grid_y,
            cells,
        }
    }

    pub fn get_grid_size(&self) -> (usize, usize, usize) {
        let cell_size = size_of::<Cell>();
        let heap_size = self.cells.len() * cell_size;
        let grid_struct_size = size_of::<Grid>();

        println!("Size of one Cell: {} bytes", cell_size);
        println!(
            "Total heap size: {} bytes (~{:.2} MB)",
            heap_size,
            heap_size as f64 / (1024.0 * 1024.0)
        );
        println!("Stack size of Grid struct: {} bytes", grid_struct_size);

        (cell_size, heap_size, grid_struct_size)
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.grid_x + x // Row-major layout: rows first, then columns
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(8);

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < self.grid_x as isize && ny >= 0 && ny < self.grid_y as isize {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }

        neighbors
    }
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
        assert_eq!(grid.cells.len(), 50);
    }

    #[test]
    fn test_grid_get_grid_size_case1() {
        let params = dummy_params(0.0);
        let grid = Grid::init(100, 100, &params);
        let (cell_size, heap_size, struct_size) = grid.get_grid_size();
        assert_eq!(cell_size, 1);
        assert_eq!(heap_size, 10000);
        assert_eq!(struct_size, 40);
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
}
