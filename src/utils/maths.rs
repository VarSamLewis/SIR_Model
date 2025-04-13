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

// Bring in types from the `grid` module.
use crate::utils::grid::{Cell, Grid, HealthState};

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

    // Loop through each cell in the grid and increment the appropriate counter
    for cell in grid.cells.iter() {
        match cell.state {
            HealthState::Susceptible => stats.susceptible += 1,
            HealthState::Infected => stats.infected += 1,
            HealthState::Recovered => stats.recovered += 1,
        }
    }

    // Return the final counts
    stats
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maths_count_states_case1() { 
        // 🧪 Create a small 2x2 grid with known health states.
        // This removes randomness and gives us full control over the expected outcome.
        let cells = vec![
            Cell { state: HealthState::Susceptible }, // 1 Susceptible
            Cell { state: HealthState::Infected },    // 1 Infected
            Cell { state: HealthState::Recovered },   // 1 Recovered
            Cell { state: HealthState::Infected },    // 1 more Infected (total: 2)
        ];

        // 🧱 Manually construct the grid with our predefined cells
        let grid = Grid {
            grid_x: 2,
            grid_y: 2,
            cells,
        };

        // 🧮 Call the function under test
        let stats = count_states(&grid);

        // ✅ Assert that the counts match what we expect
        assert_eq!(stats.susceptible, 1); // 1 Susceptible
        assert_eq!(stats.infected, 2);    // 2 Infected
        assert_eq!(stats.recovered, 1);   // 1 Recovered
    }

}