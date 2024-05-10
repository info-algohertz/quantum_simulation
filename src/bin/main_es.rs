/*
Example usage of the quantum entanglement simulation.

Example run:
cargo run --bin main_es

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::entanglement_simulation::QuantumSimulation;

fn main() {
    let qubit_count: usize = 3; // the number of qubits
    let simulation = QuantumSimulation::new(qubit_count, 0u64);
    dbg!(&simulation);
}
