/*
Example usage of the quantum simulation
including qubit initialization, quantum complete set gate application, and measurement.

Example run:
cargo run --bin main

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::simulation::QuantumSimulation;
use quantum_simulation::evaluation::evaluate;

const RUN_COUNT: usize = 100;

fn main() {
    let qubit_count: usize = 3; // the number of qubits
    let mut simulation = QuantumSimulation::new(qubit_count, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..RUN_COUNT {
        simulation.init_rnd_state();
        //dbg!(&simulation);
        simulation.pauli_x(0);
        simulation.pauli_y(1);
        simulation.pauli_z(2);
        simulation.cz(0, 1);
        simulation.toffoli(0, 1, 2);
        simulation.s(0);
        simulation.swap(1, 2);
        simulation.t(1);
        simulation.cnot(0, 1);
        simulation.hadamard(1);
        let measured_states = simulation.measure();
        measurements.push(measured_states);
        //dbg!(&simulation);
        //dbg!(measured_states);
    }
    evaluate(measurements);
}

