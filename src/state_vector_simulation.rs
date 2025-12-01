/*
Quantum full state vector simulation with a quantum complete set of gates.

This simulation evaluates all the possibilities before taking a measurement
based on the probability from a given random seed.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::gate;
use crate::simulation::Simulation;
use crate::state_vector_init::{
    ONE_QUBIT, Qubit, ZERO_QUBIT, get_amplitudes, get_ground_state_amplitudes,
};

const MAX_QUBIT_COUNT: usize = 32;

#[derive(Debug)]
pub struct QuantumSimulation {
    qubit_count: usize,
    amplitudes: Vec<Complex<f64>>,
    rng: StdRng,
}

impl QuantumSimulation {
    pub fn new(qubit_count: usize, rnd_seed: u64) -> QuantumSimulation {
        assert!(
            qubit_count <= MAX_QUBIT_COUNT,
            "The number of qubits in the simulation cannot exceed {}.",
            MAX_QUBIT_COUNT
        );

        let mut simulation = QuantumSimulation {
            qubit_count,
            amplitudes: Vec::new(),
            rng: StdRng::seed_from_u64(rnd_seed),
        };
        simulation.reset();

        simulation
    }

    fn _choose_state(&mut self) -> usize {
        let probabilities: Vec<f64> = self
            .amplitudes
            .iter()
            .map(|amplitude| amplitude.norm_sqr())
            .collect();
        let random_number: f64 = self.rng.random();
        let mut accumulated_probability = 0.0;
        let mut state_index = 0;

        for (i, &probability) in probabilities.iter().enumerate() {
            accumulated_probability += probability;
            if random_number <= accumulated_probability {
                state_index = i;
                break;
            }
        }

        state_index
    }

    fn apply_one_qubit_gate<F>(&mut self, one_qubit_gate: F, qubit_number: usize)
    where
        F: Fn(Complex<f64>, Complex<f64>) -> (Complex<f64>, Complex<f64>),
    {
        assert!(
            qubit_number < self.qubit_count,
            "The qubit number has to be less than the number of qubits {}.",
            self.qubit_count
        );

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

    fn apply_two_qubit_gate<F>(
        &mut self,
        two_qubit_gate: F,
        qubit_number0: usize,
        qubit_number1: usize,
    ) where
        F: Fn(
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
        ) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>),
    {
        assert!(
            (qubit_number0 < self.qubit_count) && (qubit_number1 < self.qubit_count),
            "The qubit number has to be less than the number of qubits {}.",
            self.qubit_count
        );

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
                let (a00, a01, a10, a11) = two_qubit_gate(
                    self.amplitudes[i00],
                    self.amplitudes[i01],
                    self.amplitudes[i10],
                    self.amplitudes[i11],
                );
                self.amplitudes[i00] = a00;
                self.amplitudes[i01] = a01;
                self.amplitudes[i10] = a10;
                self.amplitudes[i11] = a11;
            }
        }
    }

    fn apply_three_qubit_gate<F>(
        &mut self,
        three_qubit_gate: F,
        qubit_number0: usize,
        qubit_number1: usize,
        qubit_number2: usize,
    ) where
        F: Fn(
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
        ) -> (
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
            Complex<f64>,
        ),
    {
        assert!(
            (qubit_number0 < self.qubit_count)
                && (qubit_number1 < self.qubit_count)
                && (qubit_number2 < self.qubit_count),
            "The qubit number has to be less than the number of qubits {}.",
            self.qubit_count
        );

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
                let (a000, a001, a010, a011, a100, a101, a110, a111) = three_qubit_gate(
                    self.amplitudes[i000],
                    self.amplitudes[i001],
                    self.amplitudes[i010],
                    self.amplitudes[i011],
                    self.amplitudes[i100],
                    self.amplitudes[i101],
                    self.amplitudes[i110],
                    self.amplitudes[i111],
                );
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
}

impl Simulation for QuantumSimulation {
    fn reset(&mut self) {
        self.amplitudes = get_ground_state_amplitudes(self.qubit_count)
    }

    // Measure all the qubits in the Z-basis.
    fn measure_all(&mut self) -> Vec<bool> {
        let measured_state_index = self._choose_state();
        let mut measured_states: Vec<bool> = Vec::with_capacity(self.qubit_count);
        let mut qubits: Vec<Qubit<f64>> = Vec::with_capacity(self.qubit_count);
        for qubit_number in 0..self.qubit_count {
            let measured_state = measured_state_index & (1 << qubit_number) > 0;
            measured_states.push(measured_state);
            if measured_state {
                qubits.push(ONE_QUBIT);
            } else {
                qubits.push(ZERO_QUBIT);
            }
        }
        self.amplitudes = get_amplitudes(qubits);

        measured_states
    }

    // Measure the selected qubits in the Z-basis.
    fn measure(&mut self, qubit_numbers: Vec<usize>) -> Vec<bool> {
        for qubit_number in qubit_numbers.iter() {
            assert!(
                qubit_number < &self.qubit_count,
                "The qubit number has to be less than the number of qubits {}.",
                self.qubit_count
            );
        }

        let measured_state_index = self._choose_state();
        let mut measured_states: Vec<bool> = Vec::with_capacity(self.qubit_count);

        for qubit_number in qubit_numbers.iter() {
            let measured_state = measured_state_index & (1 << qubit_number) > 0;
            measured_states.push(measured_state);
        }

        let mut accumulated_probability = 0.0;
        let mut possible_amplitude_indices: Vec<usize> = Vec::with_capacity(self.amplitudes.len());
        'state_iteration: for i in 0..self.amplitudes.len() {
            for (j, &qubit_number) in qubit_numbers.iter().enumerate() {
                let qubit_state = i & (1 << qubit_number) > 0;
                if measured_states[j] != qubit_state {
                    self.amplitudes[i] = Complex::new(0.0, 0.0);
                    continue 'state_iteration;
                }
            }
            let probability = self.amplitudes[i].norm_sqr();
            accumulated_probability += probability;
            possible_amplitude_indices.push(i);
        }

        let bump_amplitude_factor = (1.0 / accumulated_probability).sqrt();
        for i in possible_amplitude_indices.into_iter() {
            self.amplitudes[i] = bump_amplitude_factor * self.amplitudes[i];
        }

        measured_states
    }

    fn pauli_x(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(gate::pauli_x, qubit_number);
    }

    fn pauli_y(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(gate::pauli_y, qubit_number);
    }

    fn pauli_z(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(gate::pauli_z, qubit_number);
    }

    fn hadamard(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(gate::hadamard, qubit_number);
    }

    fn s(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(gate::s, qubit_number);
    }

    fn t(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(gate::t, qubit_number);
    }

    fn cnot(&mut self, control_qubit_number: usize, target_qubit_number: usize) {
        self.apply_two_qubit_gate(gate::cnot, control_qubit_number, target_qubit_number);
    }

    fn cz(&mut self, control_qubit_number: usize, target_qubit_number: usize) {
        self.apply_two_qubit_gate(gate::cz, control_qubit_number, target_qubit_number);
    }

    fn swap(&mut self, qubit_number0: usize, qubit_number1: usize) {
        self.apply_two_qubit_gate(gate::swap, qubit_number0, qubit_number1);
    }

    fn toffoli(
        &mut self,
        control_qubit_number0: usize,
        control_qubit_number1: usize,
        target_qubit_number: usize,
    ) {
        self.apply_three_qubit_gate(
            gate::toffoli,
            control_qubit_number0,
            control_qubit_number1,
            target_qubit_number,
        );
    }

    fn apply_u_f<const N: usize, F>(&mut self, f: F, input_qubits: [usize; N], answer_qubit: usize)
    where
        F: Fn([bool; N]) -> bool,
    {
        let mut mask_x = [0usize; N];
        for j in 0..N {
            mask_x[j] = 1usize << input_qubits[j];
        }
        let mask_y = 1usize << answer_qubit;

        for i0 in 0..self.amplitudes.len() {
            //Get state for y. If 1, then continue.
            if i0 & mask_y != 0 {
                continue;
            }

            //Get state for x.
            let mut x = [false; N];
            for j in 0..N {
                x[j] = i0 & mask_x[j] != 0;
            }

            // Note: y XOR f(x) = y iff f(x) = 0.
            if !f(x) {
                continue;
            }

            //Swap the amplitudes of the states where y=0 and y=1.
            let i1 = i0 | mask_y;
            let a = self.amplitudes[i0];
            self.amplitudes[i0] = self.amplitudes[i1];
            self.amplitudes[i1] = a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _run_program1(simulation: &mut QuantumSimulation) {
        simulation.reset();
        simulation.pauli_x(0);
        simulation.pauli_y(1);
        simulation.pauli_z(2);
        simulation.cz(0, 1);
        simulation.toffoli(0, 1, 2);
        simulation.s(0);
        simulation.swap(1, 2);
        simulation.t(1);
        simulation.cnot(0, 1);
        simulation.hadamard(1);
    }

    #[test]
    fn measurements_are_consistent() {
        let qubit_count: usize = 3;
        let run_count: usize = 100;

        let mut simulation012 = QuantumSimulation::new(qubit_count, 0u64);
        let mut simulation1 = QuantumSimulation::new(qubit_count, 0u64);
        let mut simulation02 = QuantumSimulation::new(qubit_count, 0u64);
        for _ in 0..run_count {
            _run_program1(&mut simulation012);
            _run_program1(&mut simulation1);
            _run_program1(&mut simulation02);
            let measurements012 = simulation012.measure_all();
            let measurements1 = simulation1.measure(vec![1]);
            let measurements02 = simulation02.measure(vec![0, 2]);
            assert_eq!(measurements012[1], measurements1[0]);
            assert_eq!(measurements012[0], measurements02[0]);
            assert_eq!(measurements012[2], measurements02[1]);
        }
    }
}
