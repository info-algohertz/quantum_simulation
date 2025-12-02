/*
Bernstein-Vazirani Algorithm to find the secret dot product string of a function f.

Find s of a function f which takes x and returns
s•x = s[n-1] ∧ x[n-1] ⊕...⊕ s[0] ∧ x[0]

Example run:
cargo run --bin bernstein_vazirani

Copyright © 2025 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::Simulation;
use quantum_simulation::state_vector_simulation::QuantumSimulation;

const RUN_COUNT: usize = 100;

// Bernstein-Vazirani Algorithm to find the secret dot product string of a function f.
// Assumes the input and the answer qubits are initialized to the ground state.
// The input qubits are tha target of the measurement.
// The answer qubit is just auxiliary for U_f, not used for the measurement.
//
// Q0, Q1, Q(n-1) (input qubits):  ∣0⟩ ---------- |H| -- |+⟩ --|     |-- |H| -- = measurement
//                                                             | U_f |
// Qn (answer qubit):              ∣0⟩ -- |PX| -- |H| -- ∣-⟩ --|     |----------- ∣-⟩

fn apply_bernstein_vazirani_algo<const N: usize, F, S: Simulation>(
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

fn run_bernstein_vazirani_algo<const N: usize, F>(f: F, run_count: usize)
where
    F: Fn([bool; N]) -> bool,
{
    let mut simulation = QuantumSimulation::new(N + 1, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    let input_qubits: [usize; N] = std::array::from_fn(|i| i);
    for _ in 0..run_count {
        simulation.reset();
        let measured_states = apply_bernstein_vazirani_algo(&mut simulation, input_qubits, N, &f);
        measurements.push(measured_states);
    }

    evaluate(measurements);
}

fn dot_product<const N: usize>(input_string: [bool; N], secret_string: [bool; N]) -> bool {
    let mut product = false;
    for i in 0..N {
        product ^= input_string[i] & secret_string[i];
    }
    product
}

macro_rules! make_dot_product_fn {
    ($name:ident, $secret:expr) => {
        fn $name(input: [bool; $secret.len()]) -> bool {
            dot_product(input, $secret)
        }
    };
}

fn main() {
    const SECRET3: [bool; 3] = [true, false, true];
    make_dot_product_fn!(oracle3, SECRET3);
    println!("Bernstein-Vazirani algorithm on secret mask: {:?}", SECRET3);
    run_bernstein_vazirani_algo(oracle3, RUN_COUNT);
    println!();

    const SECRET4: [bool; 4] = [true, false, false, true];
    make_dot_product_fn!(oracle4, SECRET4);
    println!("Bernstein-Vazirani algorithm on secret mask: {:?}", SECRET4);
    run_bernstein_vazirani_algo(oracle4, RUN_COUNT);
    println!();

    const SECRET5: [bool; 5] = [true, true, false, false, true];
    make_dot_product_fn!(oracle5, SECRET5);
    println!("Bernstein-Vazirani algorithm on secret mask: {:?}", SECRET5);
    run_bernstein_vazirani_algo(oracle5, RUN_COUNT);
    println!();

    const SECRET6: [bool; 6] = [true, true, false, false, true, false];
    make_dot_product_fn!(oracle6, SECRET6);
    println!("Bernstein-Vazirani algorithm on secret mask: {:?}", SECRET6);
    run_bernstein_vazirani_algo(oracle6, RUN_COUNT);
    println!();
}
