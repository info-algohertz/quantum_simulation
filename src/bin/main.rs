/*
Example usage of the quantum simulation
including qubit initialization, quantum complete set gate application, and measurement.

Example run:
cargo run --bin main

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use rand::SeedableRng;
use rand::rngs::StdRng;

const RNG_SEED: u64 = 43;

fn main() {
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    let qubit_count: usize = 3; // the number of qubits
    let mut simulation = quantum_simulation::QuantumSimulation::new(qubit_count);
    simulation.init_rnd_state(&mut rng);
    dbg!(&simulation);
    simulation.pauli_x(0);
    simulation.pauli_y(1);
    simulation.pauli_z(2);
    simulation.cnot(0, 1);
    let measured_states = simulation.measure(&mut rng);
    println!("Quantum simulation!");
    dbg!(&simulation);
    dbg!(measured_states);
}

