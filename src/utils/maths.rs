pub struct SirParams {
    pub beta: f64,   // Infection rate
    pub gamma: f64,  // Recovery rate
    pub dt:f64,
    pub i_ratio: f64,
    pub s_ratio: f64,
}
/*
//Future use in an ODE based approach rather than an agent-based approach
pub fn update_sir(s: f64, i: f64, r: f64, params: &SirParams, dt: f64) -> (f64, f64, f64) {
    let ds = -params.beta * s * i * dt;
    let di = (params.beta * s * i - params.gamma * i) * dt;
    let dr = params.gamma * i * dt;

    (s + ds, i + di, r + dr)
}
*/
use crate::utils::grid::{Grid, HealthState};

/// Holds counts of how many people are in each state.
/// This is used to track how the disease progresses over time.
pub struct PopulationStats {
    pub susceptible: usize,
    pub infected: usize,
    pub recovered: usize,
}

/// Count how many cells are in each HealthState (S, I, or R).
/// This is useful for statistics and visualizing or logging simulation progress.
pub fn count_states(grid: &Grid) -> PopulationStats {
    // Initialize all counts to zero
    let mut stats = PopulationStats {
        susceptible: 0,
        infected: 0,
        recovered: 0,
    };

    // Iterate over every cell by linear index
    let total_cells = grid.grid_x * grid.grid_y;
    for idx in 0..total_cells {
        match grid.read(idx) {
            HealthState::Susceptible => stats.susceptible += 1,
            HealthState::Infected    => stats.infected    += 1,
            HealthState::Recovered   => stats.recovered   += 1,
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::maths::SirParams;
    use crate::utils::grid::Grid;
    use crate::utils::grid::HealthState;

    fn dummy_params(i_ratio: f64) -> SirParams {
        SirParams { beta: 0.0, gamma: 0.0, dt: 1.0, i_ratio, s_ratio: 1.0 }
    }

    #[test]
    fn test_maths_count_states_case1() {
        // Create a 2x2 grid with known states
        let mut grid = Grid::init(2, 2, &dummy_params(0.0));
        // Manually assign states
        grid.write(grid.get_index(0, 0), HealthState::Susceptible);
        grid.write(grid.get_index(1, 0), HealthState::Infected);
        grid.write(grid.get_index(0, 1), HealthState::Recovered);
        grid.write(grid.get_index(1, 1), HealthState::Infected);

        let stats = count_states(&grid);
        assert_eq!(stats.susceptible, 1);
        assert_eq!(stats.infected,    2);
        assert_eq!(stats.recovered,   1);
    }
}