/*
Quantum gates.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use num_complex::Complex;

const INV_SQRT_2: f64 = 0.7071067811865475;

pub(crate) fn pauli_x(
    amplitude0: Complex<f64>,
    amplitude1: Complex<f64>,
) -> (Complex<f64>, Complex<f64>) {
    (amplitude1, amplitude0)
}

pub(crate) fn pauli_y(
    amplitude0: Complex<f64>,
    amplitude1: Complex<f64>,
) -> (Complex<f64>, Complex<f64>) {
    (
        Complex::new(0.0, -1.0) * amplitude1,
        Complex::new(0.0, 1.0) * amplitude0,
    )
}

pub(crate) fn pauli_z(
    amplitude0: Complex<f64>,
    amplitude1: Complex<f64>,
) -> (Complex<f64>, Complex<f64>) {
    (amplitude0, -amplitude1)
}

pub(crate) fn hadamard(
    amplitude0: Complex<f64>,
    amplitude1: Complex<f64>,
) -> (Complex<f64>, Complex<f64>) {
    (
        INV_SQRT_2 * (amplitude0 + amplitude1),
        INV_SQRT_2 * (amplitude0 - amplitude1),
    )
}

pub(crate) fn s(
    amplitude0: Complex<f64>,
    amplitude1: Complex<f64>,
) -> (Complex<f64>, Complex<f64>) {
    (amplitude0, Complex::new(0.0, 1.0) * amplitude1)
}

pub(crate) fn t(
    amplitude0: Complex<f64>,
    amplitude1: Complex<f64>,
) -> (Complex<f64>, Complex<f64>) {
    (
        amplitude0,
        Complex::new(INV_SQRT_2, INV_SQRT_2) * amplitude1,
    )
}

//CX gate
pub(crate) fn cnot(
    amplitude00: Complex<f64>,
    amplitude01: Complex<f64>,
    amplitude10: Complex<f64>,
    amplitude11: Complex<f64>,
) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude00, amplitude11, amplitude10, amplitude01)
}

pub(crate) fn cz(
    amplitude00: Complex<f64>,
    amplitude01: Complex<f64>,
    amplitude10: Complex<f64>,
    amplitude11: Complex<f64>,
) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude00, amplitude01, amplitude10, -amplitude11)
}

pub(crate) fn swap(
    amplitude00: Complex<f64>,
    amplitude01: Complex<f64>,
    amplitude10: Complex<f64>,
    amplitude11: Complex<f64>,
) -> (Complex<f64>, Complex<f64>, Complex<f64>, Complex<f64>) {
    (amplitude00, amplitude10, amplitude01, amplitude11)
}

pub(crate) fn toffoli(
    amplitude000: Complex<f64>,
    amplitude001: Complex<f64>,
    amplitude010: Complex<f64>,
    amplitude011: Complex<f64>,
    amplitude100: Complex<f64>,
    amplitude101: Complex<f64>,
    amplitude110: Complex<f64>,
    amplitude111: Complex<f64>,
) -> (
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
    Complex<f64>,
) {
    (
        amplitude000,
        amplitude001,
        amplitude010,
        amplitude111,
        amplitude100,
        amplitude101,
        amplitude110,
        amplitude011,
    )
}
