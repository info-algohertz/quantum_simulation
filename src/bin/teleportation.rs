/*
Quantum teleportation example.

Transfer the full quantum information from one qubit to another qubit.

Example run:
cargo run --bin teleportation

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 3;
const RUN_COUNT: usize = 1000;

fn main() {
    println!("Teleportation:");
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..RUN_COUNT {
        simulation.init_ground_state();

        // Prepare qubit 0. It will be teleported.
        simulation.hadamard(0);

        // Entangle qubits 1 and 2.
        simulation.hadamard(1);
        simulation.cnot(1, 2);

        // Teleport the quantum information from qubit 0 into qubit 2.
        simulation.cnot(0, 1);
        simulation.hadamard(0);
        let measured_states = simulation.measure(vec![0, 1]);
        let first_bit = measured_states[0];
        let second_bit = measured_states[1];
        if first_bit {
            simulation.pauli_z(2);
        }
        if second_bit {
            simulation.pauli_x(2);
        }

        let measured_states = simulation.measure(vec![2]);
        measurements.push(measured_states);
    }
    evaluate(measurements);
}
