use rand::Rng;
use crate::utils::grid::{Grid, HealthState, Tile, tile_grid};

use crate::utils::maths::SirParams;

/// Count how many infected neighbors are around (x, y)
fn count_infected_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    grid.get_neighbors(x, y)
        .iter()
        .filter(|(nx, ny)| {
            let n_idx = grid.get_index(*nx, *ny);
            grid.read(n_idx) == HealthState::Infected
        })
        .count()
}

/// Determine if a susceptible cell should become infected
fn process_susceptible(grid: &Grid, x: usize, y: usize, params: &SirParams) -> HealthState {
    let infected_neighbors = count_infected_neighbors(grid, x, y);
    let infection_probability = (params.beta * infected_neighbors as f64 / 8.0) * params.dt;
    if rand::thread_rng().r#gen::<f64>() < infection_probability {
        HealthState::Infected
    } else {
        HealthState::Susceptible
    }
}

fn process_infected(params: &SirParams) -> HealthState {
    if rand::thread_rng().r#gen::<f64>() < params.gamma * params.dt {
        HealthState::Recovered
    } else {
        HealthState::Infected
    }
}

pub fn step_grid(grid: &mut Grid, params: &SirParams) {
    // Clone cells buffer for writing next state
    let mut new_grid = Grid {
        grid_x: grid.grid_x,
        grid_y: grid.grid_y,
        cells: grid.cells.clone(),
    };

    for y in 0..grid.grid_y {
        for x in 0..grid.grid_x {
            let idx = grid.get_index(x, y);
            let current = grid.read(idx);
            let updated = match current {
                HealthState::Susceptible => process_susceptible(grid, x, y, params),
                HealthState::Infected    => process_infected(params),
                HealthState::Recovered   => HealthState::Recovered,
            };
            new_grid.write(idx, updated);
        }
    }

    *grid = new_grid;
}
pub fn step_tile(tile: &Tile, params: &SirParams, output: &mut Grid) {
    for y in 0..tile.tile_y {
        for x in 0..tile.tile_x {
            let idx = output.get_index(tile.origin_x + x, tile.origin_y + y);
            let current = tile.get_state(x, y).unwrap();
            let neighbors = tile.get_neighbors(x, y);

            let new_state = match current {
                HealthState::Susceptible => {
                    let infected_neighbors = neighbors.iter().filter(|&&s| s == HealthState::Infected).count();
                    let p = (params.beta * infected_neighbors as f64 / 8.0) * params.dt;
                    if rand::random::<f64>() < p {
                        HealthState::Infected
                    } else {
                        HealthState::Susceptible
                    }
                }
                HealthState::Infected => {
                    if rand::random::<f64>() < params.gamma * params.dt {
                        HealthState::Recovered
                    } else {
                        HealthState::Infected
                    }
                }
                HealthState::Recovered => HealthState::Recovered,
            };

            output.write(idx, new_state);
        }
    }
}

pub fn step_grid_tiled(grid: &Grid, params: &SirParams, tile_width: usize, tile_height: usize) -> Grid {
    let mut next = Grid::init(grid.grid_x, grid.grid_y, &SirParams { beta: 0.0, gamma: 0.0, dt: 1.0, i_ratio: 0.0, s_ratio: 1.0 });
    let tiles = tile_grid(grid, tile_width, tile_height);
    for tile in &tiles {
        step_tile(tile, params, &mut next);
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::maths::SirParams;

    fn dummy_params(i_ratio: f64, beta: f64, gamma: f64, dt: f64) -> SirParams {
        SirParams { beta, gamma, dt, i_ratio, s_ratio: 1.0 }
    }

    #[test]
    // Counts 4 infected neighbors around a center cell
    fn test_simulation_count_infected_neighbors_case1() {
        let mut grid = Grid::init(3, 3, &dummy_params(0.0, 0.0, 0.0, 1.0));
        // Setup infected neighbors
        grid.write(grid.get_index(0, 0), HealthState::Infected);
        grid.write(grid.get_index(1, 0), HealthState::Infected);
        grid.write(grid.get_index(2, 1), HealthState::Infected);
        grid.write(grid.get_index(2, 2), HealthState::Infected);

        let count = count_infected_neighbors(&grid, 1, 1);
        assert_eq!(count, 4);
    }

    #[test]
    // Cell surrounded by infected neighbors should almost always get infected
    fn test_simulation_process_susceptible_case1() {
        let mut grid = Grid::init(3, 3, &dummy_params(0.0, 1.0, 0.0, 1.0));
        // all infected
        for y in 0..3 {
            for x in 0..3 {
                grid.write(grid.get_index(x, y), HealthState::Infected);
            }
        }
        let result = process_susceptible(&grid, 1, 1, &dummy_params(0.0, 1.0, 0.0, 1.0));
        assert_eq!(result, HealthState::Infected);
    }

    #[test]
    // With beta = 0.0, cell should not get infected even if surrounded
    fn test_simulation_process_susceptible_case2() {
        let grid = Grid::init(3, 3, &dummy_params(0.0, 0.0, 0.0, 1.0));
        let result = process_susceptible(&grid, 1, 1, &dummy_params(0.0, 0.0, 0.0, 1.0));
        assert_eq!(result, HealthState::Susceptible);
    }

    #[test]
    // Infected cell should always recover when gamma = 1.0
    fn test_simulation_process_infected_case1() {
        let result = process_infected(&dummy_params(0.0, 0.0, 1.0, 1.0));
        assert_eq!(result, HealthState::Recovered);
    }

    #[test]
    // Infected cell should never recover when gamma = 0.0
    fn test_simulation_process_infected_case2() {
        let result = process_infected(&dummy_params(0.0, 0.0, 0.0, 1.0));
        assert_eq!(result, HealthState::Infected);
    }

    #[test]
    // After one step, a susceptible center cell should become infected
    fn test_simulation_step_grid_case1() {
        let mut grid = Grid::init(3, 3, &dummy_params(0.0, 1.0, 0.0, 1.0));
        // all infected
        for y in 0..3 {
            for x in 0..3 {
                grid.write(grid.get_index(x, y), HealthState::Infected);
            }
        }
        // center susceptible
        grid.write(grid.get_index(1, 1), HealthState::Susceptible);

        step_grid(&mut grid, &dummy_params(0.0, 1.0, 0.0, 1.0));
        assert_eq!(grid.read(grid.get_index(1, 1)), HealthState::Infected);
    }
}