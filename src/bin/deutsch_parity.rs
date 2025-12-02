/*
Deutsch's Algorithm to solve the parity problem.

For a boolean function f:
- return 1 if and only if the number of the outputs of f that return true is even,
- return 0 otherwise.

Example run:
cargo run --bin deutsch_parity

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::Simulation;
use quantum_simulation::state_vector_simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 2;
const RUN_COUNT: usize = 100;

// Deutsch's Algorithm to find out the parity problem of the function f.
// Assumes the auxiliary and the target qubits are initialized to the ground state.
// Q0: ∣0⟩ -- |PX| -- |H| -- ∣-⟩ --|     | ----------- ∣-⟩
//                                 | U_f |
// Q1: ∣0⟩ ---------- |H| -------- |     | -- |H| -- = parity

// Q0: ∣0⟩ ---------- |H| -- |+⟩ --|     | -- |H| -- = parity, x, should be measured, target
//                                 | U_f |
// Q1: ∣0⟩ -- |PX| -- |H| -- ∣-⟩ --|     | ----------- ∣-⟩, y, aux

fn apply_deutsch_algo<S: Simulation>(
    simulation: &mut S,
    aux_qubit: usize,
    target_qubit: usize,
    f: fn(bool) -> bool,
) -> Vec<bool> {
    simulation.pauli_x(aux_qubit);
    simulation.hadamard(aux_qubit);
    simulation.hadamard(target_qubit);
    simulation.apply_u_f(move |x| f(x[0]), [target_qubit], aux_qubit);
    simulation.hadamard(target_qubit);

    simulation.measure(vec![target_qubit])
}

fn run_deutsch_algo(f: fn(bool) -> bool, run_count: usize) {
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..run_count {
        simulation.reset();
        let measured_states = apply_deutsch_algo(&mut simulation, 0, 1, f);
        measurements.push(measured_states);
    }
    evaluate(measurements);
}

fn main() {
    println!("Deutsch's algorithm on identity function");
    let id = |x: bool| -> bool { x };
    run_deutsch_algo(id, RUN_COUNT);
    println!();

    println!("Deutsch's algorithm on negation function");
    let neg = |x: bool| -> bool { !x };
    run_deutsch_algo(neg, RUN_COUNT);
    println!();

    println!("Deutsch's algorithm on a constant false function");
    let false_f = |_: bool| -> bool { false };
    run_deutsch_algo(false_f, RUN_COUNT);
    println!();

    println!("Deutsch's algorithm on a constant true function");
    let true_f = |_: bool| -> bool { true };
    run_deutsch_algo(true_f, RUN_COUNT);
}
