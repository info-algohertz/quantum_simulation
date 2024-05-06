/*
Algorithm to produce Bell state.

Bell state is a maximum entanglement state of the two qubits:
(1/sqrt(2))*(|00⟩ + |11⟩)

Example run:
cargo run --bin bell_state

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::Simulation;
use quantum_simulation::state_vector_simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 2;
const RUN_COUNT: usize = 100;

// Entangle two qubits into a Bell state (1/sqrt(2))*(|00⟩ + |11⟩).
// Assume that the qubits are initialized to the state |00⟩.
fn entangle_into_bell_state(simulation: &mut QuantumSimulation, qubit0: usize, qubit1: usize) {
    simulation.hadamard(qubit0);
    simulation.cnot(qubit0, qubit1);
}

fn main() {
    println!("Bell state:");
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..RUN_COUNT {
        simulation.init_ground_state();
        entangle_into_bell_state(&mut simulation, 0, 1);
        let measured_states = simulation.measure_all();
        measurements.push(measured_states);
    }
    evaluate(measurements);
    println!();
}
