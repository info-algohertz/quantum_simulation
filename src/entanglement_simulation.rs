/*
TODO: complete the implementation.

Quantum simulation based on simulating an entanglement with a virtual qubit.

An advantage of this simulation is that it potentially occupies less memory,
proportional only to the number of the qubits plus the number of the entanglements betweent them.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::gate;
use crate::state_vector_init::{Qubit, ZERO_QUBIT};

const MAX_QUBIT_COUNT: usize = 32768;

#[derive(Debug)]
pub struct QuantumSimulation {
    // The number of the physical qubits.
    qubit_count: usize,
    // All the virtual qubits.
    virt_qubits: Vec<Qubit<f64>>,
    // Each qubit state is defined by a probability weight and a reference to a virtual qubit.
    qubit_states: Vec<Vec<(f64, usize)>>,
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
            virt_qubits: Vec::with_capacity(qubit_count),
            qubit_states: Vec::with_capacity(qubit_count),
            rng: StdRng::seed_from_u64(rnd_seed),
        };
        simulation.reset();

        simulation
    }

    pub fn reset(&mut self) {
        self.virt_qubits.clear();
        self.qubit_states.clear();
        for i in 0..self.qubit_count {
            self.virt_qubits.push(ZERO_QUBIT);
            self.qubit_states.push(vec![(1.0, i)]);
        }
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

        for j in 0..self.qubit_states[qubit_number].len() {
            let (_, vq_ref) = self.qubit_states[qubit_number][j];
            let (a, b) = self.virt_qubits[vq_ref];
            self.virt_qubits[vq_ref] = one_qubit_gate(a, b);
        }
    }

    pub fn pauli_x(&mut self, qubit_number: usize) {
        self.apply_one_qubit_gate(gate::pauli_x, qubit_number);
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
        panic!("Not implemented!");
        assert!(
            (qubit_number0 < self.qubit_count) && (qubit_number1 < self.qubit_count),
            "The qubit number has to be less than the number of qubits {}.",
            self.qubit_count
        );
    
        /*
        let new_state0: Vec<(f64, usize)> = vec![];
        let new_state1: Vec<(f64, usize)> = vec![];
        for j0 in 0..self.qubit_states[qubit_number0].len() {
            for j1 in 0..self.qubit_states[qubit_number1].len() {
                let prob0, vq_ref0 = self.qubit_states[qubit_number0][j0];
                let prob1, vq_ref1 = self.qubit_states[qubit_number1][j1];
                let prob = prob0*prob1;
                let (a0, b0) = self.virt_qubits[vq_ref0];
                let (a1, b1) = self.virt_qubits[vq_ref1];
                two_qubit_gate
            }
        }
        */
    }

}
