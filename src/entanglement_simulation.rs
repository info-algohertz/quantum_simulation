/*
Quantum simulation based on simulating an entanglement with a virtual qubit.

An advantage of this simulation is that it potentially occupies less memory,
proportional only to the number of the qubits plus the number of the entanglements betweent hem.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::gate;
use crate::parity::create_u_f;
use crate::simulation::Simulation;
use crate::state_vector_init::{
    excited_state_qubit, get_amplitudes, ground_state_qubit, ground_state_qubits, random_qubits,
    superposition_state_qubits, Qubit,
};

#[derive(Debug)]
pub struct QuantumSimulation {
    qubit_count: usize,
    amplitudes: Vec<Complex<f64>>,
    rng: StdRng,
}

