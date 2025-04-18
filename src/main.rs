mod utils {
    pub mod grid;
    pub mod maths;
    pub mod simulation;
}

use crate::utils::grid::{Grid, HealthState, tile_grid};
use crate::utils::maths::{SirParams, count_states};
use crate::utils::simulation::{step_grid, step_grid_tiled};

// Time code execution
use std::time::Instant;
use rayon::prelude::*;

fn main() {

    let start_time = Instant::now(); // Start timing
    // 1. Define simulation parameters (including infection ratios)
    let params = SirParams {
        beta: 0.3,         // Infection rate
        gamma: 0.1,        // Recovery rate
        dt: 1.0,           // Time step (days)
        i_ratio: 0.01,     // 1% initially infected
        s_ratio: 1.0,      // All others are susceptible
    };

    // 2. Initialize grid using SirParams
    let mut grid = Grid::init(100, 100, &params);

    // 3. Run simulation loop
    
    let mut day = 0;
    loop {
        let stats = count_states(&grid);
        /*
        println!(
            "Day {:3}: Susceptible = {:5}, Infected = {:5}, Recovered = {:5}",
            day, stats.susceptible, stats.infected, stats.recovered
        );
        */
        if stats.infected == 0 {
            println!("✅ Infection has died out. Simulation complete.");
            break;
        }


        step_grid(&mut grid, &params)
;       // Parallelie approach for very large grids
        //grid = step_grid_tiled(&grid, &params, 25, 25);
       

        day += 1;

    };
    
    let elapsed = start_time.elapsed(); // Stop timing
    println!(
        "⏱️ Simulation completed in {:.2?} ({} days)",
        elapsed, day
    );


}

