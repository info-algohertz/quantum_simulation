/*
Utility functions for parity problem.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;

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
