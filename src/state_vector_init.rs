/*
Quantum state vector initialization for full state vector simulation.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;
use rand::rngs::StdRng;
use rand::Rng;
use std::f64::consts::PI;

pub type Qubit<T> = (Complex<T>, Complex<T>);

fn random_qubit(rng: &mut StdRng) -> Qubit<f64> {
    let theta0: f64 = rng.gen_range(0.0..2.0 * PI);
    let theta1: f64 = rng.gen_range(0.0..2.0 * PI);
    let theta2: f64 = rng.gen_range(0.0..2.0 * PI);
    let alpha = Complex::new(
        theta0.cos() * theta1.cos() * theta2.cos(),
        theta0.sin() * theta1.cos() * theta2.cos(),
    );
    let beta = Complex::new(theta1.sin() * theta2.cos(), theta2.sin());

    (alpha, beta)
}

pub(crate) fn random_qubits(rng: &mut StdRng, n: usize) -> Vec<Qubit<f64>> {
    let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(n);
    for _ in 0..n {
        qubits.push(random_qubit(rng));
    }

    qubits
}

pub(crate) fn ground_state_qubit() -> Qubit<f64> {
    (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))
}

pub(crate) fn ground_state_qubits(n: usize) -> Vec<Qubit<f64>> {
    let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(n);
    for _ in 0..n {
        qubits.push(ground_state_qubit());
    }

    qubits
}

pub(crate) fn excited_state_qubit() -> Qubit<f64> {
    (Complex::new(0.0, 0.0), Complex::new(1.0, 0.0))
}

pub(crate) fn superposition_state_qubits(n: usize) -> Vec<Qubit<f64>> {
    let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(n);
    let sqrt_2_inv = 1.0 / 2f64.sqrt();
    for _ in 0..n {
        qubits.push((Complex::new(sqrt_2_inv, 0.0), Complex::new(sqrt_2_inv, 0.0)));
    }

    qubits
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
