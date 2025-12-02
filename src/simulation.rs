/*
Trait for a quantum computer simulation with a quantum complete set of gates.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

pub trait Simulation {
    // Initialize the qubits into the ground state |0⟩.
    fn reset(&mut self);

    // Qubit measurement functions.
    // Measure all the qubits in the Z-basis.
    fn measure_all(&mut self) -> Vec<bool>;
    // Measure the selected qubits in the Z-basis.
    fn measure(&mut self, qubit_numbers: Vec<usize>) -> Vec<bool>;

    // 1-qubit gates.
    fn pauli_x(&mut self, qubit_number: usize);
    fn pauli_y(&mut self, qubit_number: usize);
    fn pauli_z(&mut self, qubit_number: usize);
    fn hadamard(&mut self, qubit_number: usize);
    fn s(&mut self, qubit_number: usize);
    fn t(&mut self, qubit_number: usize);

    // 2-qubit gates.
    fn cnot(&mut self, control_qubit_number: usize, target_qubit_number: usize);
    fn cz(&mut self, control_qubit_number: usize, target_qubit_number: usize);
    fn swap(&mut self, qubit_number0: usize, qubit_number1: usize);

    // 3-qubit gates.
    fn toffoli(
        &mut self,
        control_qubit_number0: usize,
        control_qubit_number1: usize,
        target_qubit_number: usize,
    );

    // Oracle gate.
    fn apply_u_f<const N_IN: usize, const N_OUT: usize, F>(
        &mut self,
        f: F,
        input_qubits: [usize; N_IN],
        answer_qubits: [usize; N_OUT],
    ) where
        F: Fn([bool; N_IN]) -> [bool; N_OUT];
}
