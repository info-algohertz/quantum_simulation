/*
Utility functions for parity problem.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;

// For a boolean function f creates a quantum gate U_f.
// To make the U_f unitary, we let it represent a function
// that takes as arguments (x, y) and returns (x, f(x) XOR y).
// U_f does not act on two separate qubits, but on a 2-qubit system.
// Hence each amplitude represents a probability of an outcome of 2 qubits.
// The basis states with their vector representation are the following:
// ∣00⟩ = (1,0,0,0)
// ∣01⟩ = (0,1,0,0)
// ∣10⟩ = (0,0,1,0)
// ∣11⟩ = (0,0,0,1)
pub(crate) fn create_u_f(
    f: fn(bool) -> bool,
) -> impl Fn(
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    move |amplitude00, amplitude01, amplitude10, amplitude11| {
        // Applying the function f to determine the behavior of U_f
        let (new_amplitude10, new_amplitude11) = if f(false) {
            (amplitude10, amplitude11) // No change for f(false) = false
        } else {
            (amplitude11, amplitude10) // Swap for f(false) = true
        };

        let (new_amplitude00, new_amplitude01) = if f(true) {
            (amplitude01, amplitude00) // Swap for f(true) = true
        } else {
            (amplitude00, amplitude01) // No change for f(true) = false
        };

        (
            new_amplitude00,
            new_amplitude01,
            new_amplitude10,
            new_amplitude11,
        )
    }
}
