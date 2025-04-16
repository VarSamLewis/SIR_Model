/*!
Author: Sam Lewis  
Purpose: Benchmark the core functions of a cellular SIR (Susceptible-Infected-Recovered) disease spread model in Rust.  
This script uses the Criterion crate to measure the performance of:
- count_infected_neighbors: How many infected neighbors a cell has
- process_susceptible: Whether a susceptible cell becomes infected
- process_infected: Whether an infected cell recovers
- step_grid: One full update of the simulation grid

This is part of my first Rust project for learning systems-level simulation and performance profiling.
*/
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import your modules
use SIR_Model::utils::grid::{Grid, HealthState, Cell};
use SIR_Model::utils::maths::SirParams;
use SIR_Model::utils::simulation::{count_infected_neighbors, process_susceptible, process_infected, step_grid};


fn dummy_params() -> SirParams {
    SirParams {
        beta: 0.5,
        gamma: 0.1,
        dt: 1.0,
        i_ratio: 0.1,
        s_ratio: 0.9,
    }
}

fn dummy_grid() -> Grid {
    Grid::init(100, 100, &dummy_params()) // 2500 cells
}

fn benchmark_count_infected_neighbors(c: &mut Criterion) {
    let grid = dummy_grid();
    c.bench_function("count_infected_neighbors", |b| {
        b.iter(|| {
            count_infected_neighbors(black_box(&grid), black_box(25), black_box(25))
        })
    });
}

fn benchmark_process_susceptible(c: &mut Criterion) {
    let grid = dummy_grid();
    let params = dummy_params();
    c.bench_function("process_susceptible", |b| {
        b.iter(|| {
            process_susceptible(black_box(&grid), black_box(25), black_box(25), black_box(&params))
        })
    });
}

fn benchmark_process_infected(c: &mut Criterion) {
    let params = dummy_params();
    c.bench_function("process_infected", |b| {
        b.iter(|| {
            process_infected(black_box(&params))
        })
    });
}

fn benchmark_step_grid(c: &mut Criterion) {
    let mut grid = dummy_grid();
    let params = dummy_params();
    c.bench_function("step_grid", |b| {
        b.iter(|| {
            step_grid(black_box(&mut grid), black_box(&params))
        })
    });
}

criterion_group!(
    benches,
    benchmark_count_infected_neighbors,
    benchmark_process_susceptible,
    benchmark_process_infected,
    benchmark_step_grid
);
criterion_main!(benches);
