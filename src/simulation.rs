/*
Simple quantum simulation library with a quantum complete set of gates.

This simulation evaluates all the possibilities before taking a measurement
based on the probability from a given random seed.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f64::consts::PI;

const MAX_QUBIT_COUNT: usize = 32;
const INV_SQRT_2: f64 = 0.7071067811865475;

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

fn hadamard_gate(amplitude0: Complex<f64>, amplitude1: Complex<f64>) -> (Complex<f64>, Complex<f64>) {
    (INV_SQRT_2*(amplitude0 + amplitude1), INV_SQRT_2*(amplitude0 - amplitude1))
}

fn s_gate(amplitude0: Complex<f64>, amplitude1: Complex<f64>) -> (Complex<f64>, Complex<f64>) {
    (amplitude0, Complex::new(0.0, 1.0)*amplitude1)
}

fn t_gate(amplitude0: Complex<f64>, amplitude1: Complex<f64>) -> (Complex<f64>, Complex<f64>) {
    (amplitude0, Complex::new(INV_SQRT_2, INV_SQRT_2)*amplitude1)
}

//CX gate
fn cnot_gate(amplitude00: Complex<f64>, amplitude01: Complex<f64>, amplitude10: Complex<f64>, amplitude11: Complex<f64>) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude00, amplitude01, amplitude11, amplitude10)
}

fn cz_gate(amplitude00: Complex<f64>, amplitude01: Complex<f64>, amplitude10: Complex<f64>, amplitude11: Complex<f64>) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude00, amplitude01, amplitude10, -amplitude11)
}

fn swap_gate(amplitude00: Complex<f64>, amplitude01: Complex<f64>, amplitude10: Complex<f64>, amplitude11: Complex<f64>) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude00, amplitude10, amplitude01, amplitude11)
}

fn toffoli_gate(amplitude000: Complex<f64>, amplitude001: Complex<f64>, amplitude010: Complex<f64>, amplitude011: Complex<f64>, amplitude100: Complex<f64>, amplitude101: Complex<f64>, amplitude110: Complex<f64>, amplitude111: Complex<f64>) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude000, amplitude001, amplitude010, amplitude011, amplitude100, amplitude101, amplitude111, amplitude110)
}

#[derive(Debug)]
pub struct QuantumSimulation {
    qubit_count: usize,
    amplitudes: Vec<Complex<f64>>,
    rng: StdRng,
}

impl QuantumSimulation {

    pub fn new(qubit_count: usize, rnd_seed: u64) -> QuantumSimulation {
        assert!(qubit_count <= MAX_QUBIT_COUNT, "The number of qubits in the simulation cannot exceed {}.", MAX_QUBIT_COUNT);

        let mut simulation = QuantumSimulation {
            qubit_count,
            amplitudes: Vec::new(),
            rng: StdRng::seed_from_u64(rnd_seed),
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

    pub fn init_rnd_state(&mut self) {
        let qubits = random_qubits(&mut self.rng, self.qubit_count);
        self.amplitudes = get_amplitudes(qubits)
    }

    fn _measure_all(&mut self) -> usize {
        let probabilities: Vec<f64> = self.amplitudes.iter()
            .map(|amplitude| amplitude.norm_sqr())
            .collect();
        let random_number = self.rng.gen::<f64>();
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

    pub fn measure_all(&mut self) -> Vec<bool> {
        let measured_state_index = self._measure_all();
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

    fn measure(&mut self, qubit_numbers: Vec<usize>) -> Vec<bool> {
        for qubit_number in qubit_numbers.iter() {
            assert!(qubit_number < &self.qubit_count, "The qubit number has to be less than the number of qubits {}.", self.qubit_count);
        }

        let mut measured_states: Vec<bool> = Vec::with_capacity(self.qubit_count);

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

    pub fn hadamard(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(hadamard_gate, qubit_number);
    }

    pub fn s(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(s_gate, qubit_number);
    }

    pub fn t(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(t_gate, qubit_number);
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

    fn apply_three_qubit_gate<F>(&mut self, three_qubit_gate: F, qubit_number0: usize,  qubit_number1: usize, qubit_number2: usize)
    where F: Fn(Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
        assert!((qubit_number0 < self.qubit_count) && (qubit_number1 < self.qubit_count) && (qubit_number2 < self.qubit_count), "The qubit number has to be less than the number of qubits {}.", self.qubit_count);

        let mask001 = 1 << qubit_number0;
        let mask010 = 1 << qubit_number1;
        let mask011 = mask001 | mask010;
        let mask100 = 1 << qubit_number2;
        let mask101 = mask001 | mask100;
        let mask110 = mask010 | mask100;
        let mask111 = mask011 | mask100;
        for i000 in 0..self.amplitudes.len() {
            let one001: bool = i000 as u64 & mask001 != 0;
            let one010: bool = i000 as u64 & mask010 != 0;
            let one100: bool = i000 as u64 & mask100 != 0;
            if !one001 && !one010 && !one100 {
                let i001 = i000 + mask001 as usize;
                let i010 = i000 + mask010 as usize;
                let i011 = i000 + mask011 as usize;
                let i100 = i000 + mask100 as usize;
                let i101 = i000 + mask101 as usize;
                let i110 = i000 + mask110 as usize;
                let i111 = i000 + mask111 as usize;
                let (a000, a001, a010, a011, a100, a101, a110, a111) = three_qubit_gate(self.amplitudes[i000], self.amplitudes[i001], self.amplitudes[i010], self.amplitudes[i011], self.amplitudes[i100], self.amplitudes[i101], self.amplitudes[i110], self.amplitudes[i111]);
                self.amplitudes[i000] = a000;
                self.amplitudes[i001] = a001;
                self.amplitudes[i010] = a010;
                self.amplitudes[i011] = a011;
                self.amplitudes[i100] = a100;
                self.amplitudes[i101] = a101;
                self.amplitudes[i110] = a110;
                self.amplitudes[i111] = a111;
            }
        }
    }

    pub fn cnot(&mut self, control_qubit_number: usize, target_qubit_number: usize) {
        self.apply_two_qubit_gate(cnot_gate, control_qubit_number, target_qubit_number);
    }

    pub fn cz(&mut self, control_qubit_number: usize, target_qubit_number: usize) {
        self.apply_two_qubit_gate(cz_gate, control_qubit_number, target_qubit_number);
    }

    pub fn swap(&mut self, qubit_number0: usize, qubit_number1: usize) {
        self.apply_two_qubit_gate(swap_gate, qubit_number0, qubit_number1);
    }

    pub fn toffoli(&mut self, control_qubit_number0: usize, control_qubit_number1: usize, target_qubit_number: usize) {
        self.apply_three_qubit_gate(toffoli_gate, control_qubit_number0, control_qubit_number1, target_qubit_number);
    }
}
