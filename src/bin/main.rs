/*
Example usage of the quantum simulation
including qubit initialization, quantum complete set gate application, and measurement.

Example run:
cargo run --bin main

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::simulation::QuantumSimulation;

fn main() {
    let qubit_count: usize = 3; // the number of qubits
    let mut simulation = QuantumSimulation::new(qubit_count, 0u64);
    simulation.init_rnd_state();
    dbg!(&simulation);
    simulation.pauli_x(0);
    simulation.pauli_y(1);
    simulation.pauli_z(2);
    simulation.cnot(0, 1);
    let measured_states = simulation.measure();
    println!("Quantum simulation!");
    dbg!(&simulation);
    dbg!(measured_states);
}

