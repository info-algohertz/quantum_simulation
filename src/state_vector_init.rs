/*
Quantum state vector initialization for full state vector simulation.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;

pub type Qubit<T> = (Complex<T>, Complex<T>);

pub(crate) fn ground_state_qubit() -> Qubit<f64> {
    (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))
}

pub(crate) fn excited_state_qubit() -> Qubit<f64> {
    (Complex::new(0.0, 0.0), Complex::new(1.0, 0.0))
}

// Return the amplitudes for case with the only possible state |0...0⟩.
pub(crate) fn get_ground_state_amplitudes(qubit_count: usize) -> Vec<Complex<f64>> {
    let state_count: usize = 2usize.pow(qubit_count as u32);
    let mut amplitudes: Vec<Complex<f64>> = Vec::with_capacity(state_count);
    amplitudes.push(Complex::new(1.0, 0.0));
    for _ in 1..state_count {
        amplitudes.push(Complex::new(0.0, 0.0));
    }

    amplitudes
}

pub(crate) fn get_amplitudes(qubits: Vec<Qubit<f64>>) -> Vec<Complex<f64>> {
    let state_count: usize = 2usize.pow(qubits.len() as u32);
    let mut amplitudes: Vec<Complex<f64>> = Vec::with_capacity(state_count);

    let mut mask: Vec<u64> = Vec::with_capacity(qubits.len());
    for j in 0..qubits.len() {
        mask.push(1 << j);
    }

    for i in 0..state_count {
        let mut amplitude: Complex<f64> = Complex::new(1.0, 0.0);
        for j in 0..qubits.len() {
            let one: bool = i as u64 & mask[j] != 0;
            if one {
                amplitude *= qubits[j].1;
            } else {
                amplitude *= qubits[j].0;
            }
        }
        amplitudes.push(amplitude);
    }

    amplitudes
}
