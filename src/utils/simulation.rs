use rand::Rng;
use crate::utils::grid::{Grid, HealthState};

use crate::utils::maths::SirParams;

/// Count how many infected neighbors are around (x, y)
fn count_infected_neighbors(grid: &Grid, x: usize, y: usize) -> usize {
    grid.get_neighbors(x, y)
        .iter()
        .filter(|(nx, ny)| {
            let n_idx = grid.get_index(*nx, *ny);
            grid.cells[n_idx].state == HealthState::Infected
        })
        .count()
}

/// Determine if a susceptible cell should become infected
fn process_susceptible(grid: &Grid, x: usize, y: usize, params: &SirParams) -> HealthState {
    let infected_neighbors = count_infected_neighbors(grid, x, y);

    let infection_probability = (params.beta * infected_neighbors as f64 / 8.0) * params.dt;
    let mut rng = rand::thread_rng();

    if rng.r#gen::<f64>() < infection_probability {
        HealthState::Infected
    } else {
        HealthState::Susceptible
    }
}

fn process_infected(params: &SirParams) -> HealthState {
    //Applies the state change from process_suceptible
    let mut rng = rand::thread_rng();
    if rng.r#gen::<f64>() < params.gamma * params.dt {
        HealthState::Recovered
    } else {
        HealthState::Infected
    }
}

pub fn step_grid(grid: &mut Grid, params: &SirParams) {
    //Loop through all cells
    let mut new_cells = grid.cells.clone(); // working copy

    for y in 0..grid.grid_y {
        for x in 0..grid.grid_x {
            let idx = grid.get_index(x, y);
            let cell = grid.cells[idx];

            new_cells[idx].state = match cell.state {
                HealthState::Susceptible => process_susceptible(grid, x, y, params),
                HealthState::Infected => process_infected(params),
                HealthState::Recovered => HealthState::Recovered,
            };
        }
    }

    grid.cells = new_cells;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::grid::{Grid, Cell, HealthState};
    use crate::utils::maths::SirParams;

    // Reusable factory for SirParams with defaults
    fn dummy_params(beta: f64, gamma: f64, dt: f64) -> SirParams {
        SirParams {
            beta,
            gamma,
            dt,
            i_ratio: 0.0,
            s_ratio: 1.0,
        }
    }

    #[test]
    // Counts 4 infected neighbors around a center cell
    fn test_simulation_count_infected_neighbors_case1() {
        let cells = vec![
            Cell { state: HealthState::Infected },     // (0,0)
            Cell { state: HealthState::Infected },     // (1,0)
            Cell { state: HealthState::Susceptible },  // (2,0)
            Cell { state: HealthState::Recovered },    // (0,1)
            Cell { state: HealthState::Susceptible },  // (1,1) <- center
            Cell { state: HealthState::Infected },     // (2,1)
            Cell { state: HealthState::Susceptible },  // (0,2)
            Cell { state: HealthState::Susceptible },  // (1,2)
            Cell { state: HealthState::Infected },     // (2,2)
        ];

        let grid = Grid {
            grid_x: 3,
            grid_y: 3,
            cells,
        };

        let count = count_infected_neighbors(&grid, 1, 1);
        assert_eq!(count, 4);
    }

    #[test]
    // Cell surrounded by infected neighbors should almost always get infected
    fn test_simulation_process_susceptible_case1() {
        let cells = vec![Cell { state: HealthState::Infected }; 9];
        let grid = Grid {
            grid_x: 3,
            grid_y: 3,
            cells,
        };

        let params = dummy_params(1.0, 0.0, 1.0);
        let result = process_susceptible(&grid, 1, 1, &params);
        assert_eq!(result, HealthState::Infected);
    }

    #[test]
    // With beta = 0.0, cell should not get infected even if surrounded
    fn test_simulation_process_susceptible_case2() {
        let cells = vec![Cell { state: HealthState::Susceptible }; 9];
        let grid = Grid {
            grid_x: 3,
            grid_y: 3,
            cells,
        };

        let params = dummy_params(0.0, 0.0, 1.0);
        let result = process_susceptible(&grid, 1, 1, &params);
        assert_eq!(result, HealthState::Susceptible);
    }

    #[test]
    // Infected cell should always recover when gamma = 1.0
    fn test_simulation_process_infected_case1() {
        let params = dummy_params(0.0, 1.0, 1.0);
        let result = process_infected(&params);
        assert_eq!(result, HealthState::Recovered);
    }

    #[test]
    // Infected cell should never recover when gamma = 0.0
    fn test_simulation_process_infected_case2() {
        let params = dummy_params(0.0, 0.0, 1.0);
        let result = process_infected(&params);
        assert_eq!(result, HealthState::Infected);
    }

    #[test]
    // After one step, a susceptible center cell should become infected
    fn test_simulation_step_grid_case1() {
        let mut cells = vec![Cell { state: HealthState::Infected }; 9];
        cells[4] = Cell { state: HealthState::Susceptible }; // center

        let mut grid = Grid {
            grid_x: 3,
            grid_y: 3,
            cells,
        };

        let params = dummy_params(1.0, 0.0, 1.0);
        step_grid(&mut grid, &params);

        assert_eq!(grid.cells[4].state, HealthState::Infected);
    }
}
