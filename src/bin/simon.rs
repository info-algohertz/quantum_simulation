/*
Simon's Algorithm to find the secret XOR mask.

Find s of a function f satisfying:
f(x) = f(y) iff x ⊕ y = s for all x, y.

Example run:
cargo run --bin simon

Copyright © 2025 AlgoHertz. All rights reserved.
*/

use quantum_simulation::evaluation::evaluate;
use quantum_simulation::simulation::Simulation;
use quantum_simulation::state_vector_simulation::QuantumSimulation;

const RUN_COUNT: usize = 100;

// Simon's Algorithm to find the secret XOR mask s of a function f.
// Assumes the input and the answer qubits are initialized to the ground state.
// Both the input and answer qubits are tha targets of the measurement.
//
// Q0, ..., Q(n-1) (input qubits):    ∣0⟩ -- |H| -- |+⟩ --|     |-- |H| -- = measurement
//                                                        | U_f |
// Qn, ..., Q(2n-1) (answers qubits): ∣0⟩ ----------------|     |--------- = measurement

fn apply_simons_algo<const N: usize, F, S: Simulation>(
    simulation: &mut S,
    input_qubits: [usize; N],
    answer_qubits: [usize; N],
    f: &F,
) -> Vec<bool>
where
    F: Fn([bool; N]) -> [bool; N],
{
    for i in 0..N {
        simulation.hadamard(input_qubits[i]);
    }
    simulation.apply_u_f(&f, input_qubits, answer_qubits);
    for i in 0..N {
        simulation.hadamard(input_qubits[i]);
    }

    simulation.measure(Vec::from(input_qubits))
}

fn run_simons_algo<const N: usize, F>(f: F, run_count: usize)
where
    F: Fn([bool; N]) -> [bool; N],
{
    let mut simulation = QuantumSimulation::new(2 * N, 0u64);
    let mut measurements = Vec::with_capacity(RUN_COUNT);
    let input_qubits: [usize; N] = std::array::from_fn(|i| i);
    let answer_qubits: [usize; N] = std::array::from_fn(|i| N + i);
    for _ in 0..run_count {
        simulation.reset();
        let measured_states = apply_simons_algo(&mut simulation, input_qubits, answer_qubits, &f);
        measurements.push(measured_states);
    }

    evaluate(measurements);
}

fn xor<const N: usize>(x: [bool; N], y: [bool; N]) -> [bool; N] {
    let mut z = [false; N];
    for i in 0..N {
        z[i] = x[i] ^ y[i];
    }
    z
}

fn id<const N: usize>(x: [bool; N]) -> [bool; N] {
    x
}

fn not<const N: usize>(x: [bool; N]) -> [bool; N] {
    let mut y = [false; N];
    for i in 0..N {
        y[i] = !x[i];
    }
    y
}

fn shift<const N: usize>(x: [bool; N], n: usize) -> [bool; N] {
    let mut y = [false; N];
    for i in 0..N {
        y[(i + n) % N] = x[i];
    }
    y
}

fn function<const N: usize, F>(x: [bool; N], s: [bool; N], f: F) -> [bool; N]
where
    F: Fn([bool; N]) -> [bool; N],
{
    let y = xor(x, s);
    if x < y { f(x) } else { f(y) }
}

fn main() {
    const SECRET3: [bool; 3] = [true, false, true];
    let oracle3 = move |x| function(x, SECRET3, &id);
    println!("Simon's algorithm on secret mask: {:?}", SECRET3);
    run_simons_algo(oracle3, RUN_COUNT);
    println!();

    const SECRET4: [bool; 4] = [true, true, false, true];
    let oracle4 = move |x| function(x, SECRET4, &not);
    println!("Simon's algorithm on secret mask: {:?}", SECRET4);
    run_simons_algo(oracle4, RUN_COUNT);
    println!();

    const SECRET5: [bool; 5] = [true, false, false, true, true];
    let oracle5 = move |x| function(x, SECRET5, |x2| shift(x2, 1));
    println!("Simon's algorithm on secret mask: {:?}", SECRET5);
    run_simons_algo(oracle5, RUN_COUNT);
    println!();
}
