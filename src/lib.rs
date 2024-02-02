/*
Simple quantum simulation library with a quantum complete set of gates.

This simulation evaluates all the possibilities before taking a measurement
based on the probability from a given random seed.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;
use rand::Rng;
use rand::rngs::StdRng;
use std::f64::consts::PI;

const MAX_QUBIT_COUNT: usize = 32;

pub type Qubit<T> = (Complex<T>, Complex<T>);

fn random_qubit(rng: &mut StdRng) -> Qubit<f64> {
    let theta0: f64 = rng.gen_range(0.0..2.0 * PI);
    let theta1: f64 = rng.gen_range(0.0..2.0 * PI);
    let theta2: f64 = rng.gen_range(0.0..2.0 * PI);
    let alpha = Complex::new(
        theta0.cos()*theta1.cos()*theta2.cos(),
        theta0.sin()*theta1.cos()*theta2.cos());
    let beta = Complex::new(theta1.sin()*theta2.cos(), theta2.sin());

    (alpha, beta)
}

fn random_qubits(rng: &mut StdRng, n: usize) -> Vec<Qubit<f64>> {
    let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(n);
    for _ in 0..n {
        qubits.push(random_qubit(rng));
    }

    qubits
}

fn ground_state_qubit() -> Qubit<f64> {
    (Complex::new(1.0, 0.0), Complex::new(0.0, 0.0))
}

fn ground_state_qubits(n: usize) -> Vec<Qubit<f64>> {
    let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(n);
    for _ in 0..n {
        qubits.push(ground_state_qubit());
    }

    qubits
}

fn excited_state_qubit() -> Qubit<f64> {
    (Complex::new(0.0, 0.0), Complex::new(1.0, 0.0))
}

fn superposition_state_qubits(n: usize) -> Vec<Qubit<f64>> {
    let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(n);
    let sqrt_2_inv = 1.0 / 2f64.sqrt();
    for _ in 0..n {
        qubits.push((Complex::new(sqrt_2_inv, 0.0), Complex::new(sqrt_2_inv, 0.0)));
    }

    qubits
}

fn get_amplitudes(qubits: Vec<Qubit<f64>>) -> Vec<Complex<f64>> {
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

fn pauli_x_gate(amplitude0: Complex<f64>, amplitude1: Complex<f64>) -> (Complex<f64>, Complex<f64>) {
    (amplitude1, amplitude0)
}

fn pauli_y_gate(amplitude0: Complex<f64>, amplitude1: Complex<f64>) -> (Complex<f64>, Complex<f64>) {
    (Complex::new(0.0, -1.0)*amplitude1, Complex::new(0.0, 1.0)*amplitude0)
}

fn pauli_z_gate(amplitude0: Complex<f64>, amplitude1: Complex<f64>) -> (Complex<f64>, Complex<f64>) {
    (amplitude0, -amplitude1)
}

fn cnot_gate(amplitude00: Complex<f64>, amplitude01: Complex<f64>, amplitude10: Complex<f64>, amplitude11: Complex<f64>) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude00, amplitude01, amplitude11, amplitude10)
}

#[derive(Debug)]
pub struct QuantumSimulation {
    qubit_count: usize,
    amplitudes: Vec<Complex<f64>>,
}

impl QuantumSimulation {

    pub fn new(qubit_count: usize) -> QuantumSimulation {
        assert!(qubit_count <= MAX_QUBIT_COUNT, "The number of qubits in the simulation cannot exceed {}.", MAX_QUBIT_COUNT);
        let mut simulation = QuantumSimulation {
            qubit_count,
            amplitudes: Vec::new(),
        };
        simulation.init_ground_state();

        simulation
    }

    pub fn init_ground_state(&mut self) {
        let qubits = ground_state_qubits(self.qubit_count);
        self.amplitudes = get_amplitudes(qubits);
    }

    pub fn init_superposition_state(&mut self) {
        let qubits = superposition_state_qubits(self.qubit_count);
        self.amplitudes = get_amplitudes(qubits);
    }

    pub fn init_rnd_state(&mut self, rng: &mut StdRng) {
        let qubits = random_qubits(rng, self.qubit_count);
        self.amplitudes = get_amplitudes(qubits)
    }

    fn _measure(&self, rng: &mut StdRng) -> usize {
        let probabilities: Vec<f64> = self.amplitudes.iter()
            .map(|amplitude| amplitude.norm_sqr())
            .collect();
        let random_number = rng.gen::<f64>();
        let mut accumulated_probability = 0.0;
        let mut measured_state_index = 0;

        for (i, &probability) in probabilities.iter().enumerate() {
            accumulated_probability += probability;
            if random_number <= accumulated_probability {
                measured_state_index = i;
                break;
            }
        }

        measured_state_index
    }

    pub fn measure(&mut self, rng: &mut StdRng) -> Vec<bool> {
        let measured_state_index = self._measure(rng);
        let mut measured_states: Vec<bool> = Vec::with_capacity(self.qubit_count);
        let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(self.qubit_count);
        for i in 0..self.qubit_count {
            let measured_state = measured_state_index & (1 << i) > 0;
            measured_states.push(measured_state);
            if measured_state {
                qubits.push(excited_state_qubit());
            } else {
                qubits.push(ground_state_qubit());
            }

        }
        self.amplitudes = get_amplitudes(qubits);

        measured_states
    }

    fn apply_one_qubit_gate<F>(&mut self, one_qubit_gate: F, qubit_number: usize)
    where F: Fn(Complex<f64>, Complex<f64>) -> (Complex<f64>, Complex<f64>) {
        assert!(qubit_number < self.qubit_count, "The qubit number has to be less than the number of qubits {}.", self.qubit_count);

        let mask = 1 << qubit_number;
        for i0 in 0..self.amplitudes.len() {
            let one: bool = i0 as u64 & mask != 0;
            if !one {
                let i1 = i0 + mask as usize;
                let (a0, a1) = one_qubit_gate(self.amplitudes[i0], self.amplitudes[i1]);
                self.amplitudes[i0] = a0;
                self.amplitudes[i1] = a1;
            }
        }
    }

    pub fn pauli_x(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(pauli_x_gate, qubit_number);
    }

    pub fn pauli_y(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(pauli_y_gate, qubit_number);
    }

    pub fn pauli_z(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(pauli_z_gate, qubit_number);
    }

    fn apply_two_qubit_gate<F>(&mut self, two_qubit_gate: F, qubit_number0: usize,  qubit_number1: usize)
    where F: Fn(Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
        assert!((qubit_number0 < self.qubit_count) && (qubit_number1 < self.qubit_count), "The qubit number has to be less than the number of qubits {}.", self.qubit_count);

        let mask01 = 1 << qubit_number0;
        let mask10 = 1 << qubit_number1;
        let mask11 = mask01 | mask10;
        for i00 in 0..self.amplitudes.len() {
            let one01: bool = i00 as u64 & mask01 != 0;
            let one10: bool = i00 as u64 & mask10 != 0;
            if !one01 && !one10 {
                let i01 = i00 + mask01 as usize;
                let i10 = i00 + mask10 as usize;
                let i11 = i00 + mask11 as usize;
                let (a00, a01, a10, a11) = two_qubit_gate(self.amplitudes[i00], self.amplitudes[i01], self.amplitudes[i10], self.amplitudes[i11]);
                self.amplitudes[i00] = a00;
                self.amplitudes[i01] = a01;
                self.amplitudes[i10] = a10;
                self.amplitudes[i11] = a11;
            }
        }
    }

    pub fn cnot(&mut self, control_qubit_number: usize, target_qubit_number: usize) {
        self.apply_two_qubit_gate(cnot_gate, control_qubit_number, target_qubit_number);
    }
}
