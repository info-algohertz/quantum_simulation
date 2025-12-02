/*
Deutsch-Jozsa Algorithm to solve the problem of determining where a function is constant or balanced.

For a boolean function f, we assume that one of the following holds:
- the function is constant 0.
- the function is constant 1.
- the function is balanced: returns 0s for a half of inputs, 1s for a half of inputs.

Example run:
cargo run --bin deutsch_jozsa_balanced

Copyright © 2025 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::Simulation;
use quantum_simulation::state_vector_simulation::QuantumSimulation;

const RUN_COUNT: usize = 100;

// Deutsch-Jozsa Algorithm to solve the balancing problem for the function f.
// Assumes the input and the answer qubits are initialized to the ground state.
// The input qubits are tha target of the measurement.
// The answer qubit is just auxiliary for U_f, not used for the measurement.
//
// Q0, Q1, Q(n-1) (input qubits):  ∣0⟩ ---------- |H| -- |+⟩ --|     |-- |H| -- = measurement
//                                                             | U_f |
// Qn (answer qubit):              ∣0⟩ -- |PX| -- |H| -- ∣-⟩ --|     |----------- ∣-⟩

fn apply_deutsch_jozsa_algo<const N: usize, F, S: Simulation>(
    simulation: &mut S,
    input_qubits: [usize; N],
    answer_qubit: usize,
    f: &F,
) -> Vec<bool>
where
    F: Fn([bool; N]) -> bool,
{
    simulation.pauli_x(answer_qubit);
    simulation.hadamard(answer_qubit);
    for i in 0..N {
        simulation.hadamard(input_qubits[i]);
    }
    simulation.apply_u_f(f, input_qubits, answer_qubit);
    for i in 0..N {
        simulation.hadamard(input_qubits[i]);
    }

    simulation.measure(Vec::from(input_qubits))
}

fn run_deutsch_jozsa_algo<const N: usize, F>(f: F, run_count: usize)
where
    F: Fn([bool; N]) -> bool,
{
    let mut simulation = QuantumSimulation::new(N + 1, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    let input_qubits: [usize; N] = std::array::from_fn(|i| i);
    for _ in 0..run_count {
        simulation.reset();
        let measured_states = apply_deutsch_jozsa_algo(&mut simulation, input_qubits, N, &f);
        measurements.push(measured_states);
    }
    evaluate(measurements);
}

fn true_function(_x: [bool; 4]) -> bool {
    true
}

fn main() {
    println!("Deutsch-Jozsa algorithm on constant 1 function");
    run_deutsch_jozsa_algo(true_function, RUN_COUNT);
    println!();
}
