/*
Quantum teleportation example.

Transfer the full quantum information from one qubit to another qubit.

Example run:
cargo run --bin teleportation

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::Simulation;
use quantum_simulation::state_vector_simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 3;
const RUN_COUNT: usize = 1000;

// Teleport the quantum information from the source qubit into the destination qubit using an auxiliary qubit.
fn teleport<S: Simulation>(
    simulation: &mut S,
    src_qubit: usize,
    aux_qubit: usize,
    dest_qubit: usize,
) {
    // Entangle qubits 1 and 2.
    simulation.hadamard(aux_qubit);
    simulation.cnot(aux_qubit, dest_qubit);

    // Teleport the quantum information from qubit 0 into qubit 2.
    simulation.cnot(src_qubit, aux_qubit);
    simulation.hadamard(src_qubit);
    let measured_states = simulation.measure(vec![src_qubit, aux_qubit]);
    let first_bit = measured_states[0];
    let second_bit = measured_states[1];
    if first_bit {
        simulation.pauli_z(dest_qubit);
    }
    if second_bit {
        simulation.pauli_x(dest_qubit);
    }
}

fn main() {
    println!("Teleportation:");
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..RUN_COUNT {
        simulation.reset();

        // Prepare qubit 0. It will be teleported.
        simulation.hadamard(0);

        teleport(&mut simulation, 0, 1, 2);

        let measured_states = simulation.measure(vec![2]);
        measurements.push(measured_states);
    }
    evaluate(measurements);
}
