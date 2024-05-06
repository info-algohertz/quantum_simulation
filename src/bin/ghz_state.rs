/*
Algorithm to produce GHZ (Greenberger-Horne-Zeilinger) state.

GHZ state is a maximum entanglement state of the three qubits:
(1/sqrt(2))*(|000⟩ + |111⟩)

Example run:
cargo run --bin ghz_state

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::Simulation;
use quantum_simulation::state_vector_simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 3;
const RUN_COUNT: usize = 100;

// Entangle three qubits into a GHZ state (1/sqrt(2))*(|000⟩ + |111⟩).
// Assume that the qubits are initialized to the state |000⟩.
fn entangle_into_ghz_state(
    simulation: &mut QuantumSimulation,
    qubit0: usize,
    qubit1: usize,
    qubit2: usize,
) {
    simulation.hadamard(qubit0);
    simulation.cnot(qubit0, qubit1);
    simulation.cnot(qubit1, qubit2);
}

fn main() {
    println!("GHZ state:");
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..RUN_COUNT {
        simulation.init_ground_state();
        entangle_into_ghz_state(&mut simulation, 0, 1, 2);
        let measured_states = simulation.measure_all();
        measurements.push(measured_states);
    }
    evaluate(measurements);

    println!();
}
