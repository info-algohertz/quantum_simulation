/*
Quantum simulation based on simulating an entanglement with a virtual qubit.

An advantage of this simulation is that it potentially occupies less memory,
proportional only to the number of the qubits plus the number of the entanglements betweent hem.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;
use rand::rngs::StdRng;
use rand::SeedableRng;

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

}
