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
use quantum_simulation::simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 2;
const RUN_COUNT: usize = 100;

fn run_deutsch_algo(f: fn(bool) -> bool, run_count: usize) {
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    for _ in 0..run_count {
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
