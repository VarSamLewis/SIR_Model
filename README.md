# SimpleSIR
# Disease Simulation in Rust (SIR Model)

This is a basic disease spread simulation using the **SIR (Susceptible-Infected-Recovered)** model, implemented in **Rust**. It is an agent-based, grid-driven version of the classic epidemiological model.  
> This is my first project in Rust — designed to explore language fundamentals, performance, and structuring scientific simulations.

---

## Assumptions

- Individuals (grid cells) do not move.
- The simulation uses **8-connected neighbors** to model interactions.
- A recovered person cannot be re-infected.
- Infection and recovery follow **probabilistic rules**.
- The simulation runs in **discrete time steps** (`dt`).
- All cells are either Susceptible or Infected at initialization (no Recovered).

---

## Modeling Approach

This project models the **SIR process** using an **agent-based simulation**, where each individual is represented as a `Cell` on a 2D grid. Unlike continuous models using differential equations, this approach uses **local interactions** between neighbors.

Each `Cell` exists in one of three states:
- `Susceptible`
- `Infected`
- `Recovered`

### Key rules:
- A `Susceptible` cell becomes `Infected` based on:
  - Infection rate (`β`)
  - Time step (`dt`)
  - Number of infected neighbors (out of 8 max)
- An `Infected` cell becomes `Recovered` based on:
  - Recovery rate (`γ`)
  - Time step (`dt`)

The simulation ends when no `Infected` cells remain.

---
## Project Structure

src/
├── main.rs              # Simulation runner
└── utils/
    ├── grid.rs          # Grid and neighbor logic
    ├── maths.rs         # Parameters and SIR logic
    └── simulation.rs    # Time-step update logic (step_grid)

## Testing
Unit tests are written for each module.

Tests include:
- Grid dimensions and index math
- State counting
- Neighbor detection
- Infection and recovery mechanics

## Possible Extensions

- Add CSV output for plotting results
- Add GUI or visualizer using a crate like egui
- Add CLI interface (e.g. clap) for configuring parameters
- Add more compartments: SEIR, SIS, etc.
- Introduce mobility or vaccination dynamics


