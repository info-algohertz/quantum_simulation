/*
Algorithm to produce Bell state.

Bell state is a maximum entanglement state of the two qubits:
(1/sqrt(2))*(|00⟩ + |11⟩)

Example run:
cargo run --bin bell_state

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 2;
const RUN_COUNT: usize = 100;

fn produce_ghz_state(run_count: usize) {
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..run_count {
        simulation.init_ground_state();
        simulation.hadamard(0);
        simulation.cnot(0, 1);
        let measured_states = simulation.measure_all();
        measurements.push(measured_states);
    }
    evaluate(measurements);
}

fn main() {
    println!("Bell state:");
    produce_ghz_state(RUN_COUNT);
    println!();
}
