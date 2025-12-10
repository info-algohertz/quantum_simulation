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
// Q0, ..., Q(n-1) (input qubits):  ∣0⟩ ---------- |H| -- |+⟩ --|     |-- |H| -- = measurement
//                                                              | U_f |
// Qn (answer qubit):               ∣0⟩ -- |PX| -- |H| -- ∣-⟩ --|     |----------- ∣-⟩

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
    simulation.apply_u_f(move |x| [f(x)], input_qubits, [answer_qubit]);
    for i in 0..N {
        simulation.hadamard(input_qubits[i]);
    }

    simulation.measure(Vec::from(input_qubits))
}

fn is_constant(measurements: &Vec<bool>) -> bool {
    for i in 0..measurements.len() {
        if measurements[i] {
            return false;
        }
    }
    true
}

fn run_deutsch_jozsa_algo<const N: usize, F>(f: F, run_count: usize)
where
    F: Fn([bool; N]) -> bool,
{
    let mut simulation = QuantumSimulation::new(N + 1, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    let input_qubits: [usize; N] = std::array::from_fn(|i| i);
    let mut constant_count = 0usize;
    for _ in 0..run_count {
        simulation.reset();
        let measured_states = apply_deutsch_jozsa_algo(&mut simulation, input_qubits, N, &f);
        if is_constant(&measured_states) {
            constant_count += 1;
        }
        measurements.push(measured_states);
    }

    if constant_count == run_count {
        println!("The function is constant.");
    } else if constant_count == 0 {
        println!("The function is balanced.");
    } else {
        // Such function can still end up with some small probability in the constant or balanced category.
        println!("The function is neither constant nor balanced.");
    }
    evaluate(measurements);
}

fn false_function(_x: [bool; 4]) -> bool {
    false
}

fn true_function(_x: [bool; 8]) -> bool {
    true
}

fn projection_function(x: [bool; 6]) -> bool {
    x[0]
}

fn xor_function(x: [bool; 4]) -> bool {
    x[0] ^ x[1] ^ x[2] ^ x[3]
}

fn and_function(x: [bool; 4]) -> bool {
    x[0] & x[1] & x[2] & x[3]
}

fn main() {
    println!("Deutsch-Jozsa algorithm on Constant 0 function");
    run_deutsch_jozsa_algo(false_function, RUN_COUNT);
    println!();

    println!("Deutsch-Jozsa algorithm on Constant 1 function");
    run_deutsch_jozsa_algo(true_function, RUN_COUNT);
    println!();

    println!("Deutsch-Jozsa algorithm on Projection function");
    run_deutsch_jozsa_algo(projection_function, RUN_COUNT);
    println!();

    println!("Deutsch-Jozsa algorithm on XOR function");
    run_deutsch_jozsa_algo(xor_function, RUN_COUNT);
    println!();

    println!("Deutsch-Jozsa algorithm on AND function");
    run_deutsch_jozsa_algo(and_function, RUN_COUNT);
    println!();
}
