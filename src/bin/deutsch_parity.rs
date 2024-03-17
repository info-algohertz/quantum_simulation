/*
Deutsch's Algorithm to solve the parity problem.

Example run:
cargo run --bin deutsch_parity

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::simulation::QuantumSimulation;
use quantum_simulation::evaluation::evaluate;

const RUN_COUNT: usize = 100;

fn main() {
    let qubit_count: usize = 2;
    let f = |x: bool| -> bool { x }; // Identity function
    let mut simulation = QuantumSimulation::new(qubit_count, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..RUN_COUNT {
        simulation.init_ground_state();
        simulation.pauli_x(0);
        simulation.hadamard(0);
        simulation.hadamard(1);
        simulation.apply_u_f(f, 0, 1);
        simulation.hadamard(1);
        let measured_states = simulation.measure(vec![1]);
        measurements.push(measured_states);
    }
    evaluate(measurements);
}

