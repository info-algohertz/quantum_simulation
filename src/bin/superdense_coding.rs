/*
Superdense coding example.

Send two bits of information by sending only one quantum bit.
The reduction in the amount of the information needed to be sent
is possible due to the arrangement that at the beginning both
parties each have a qubit and these two qubits are entangled.
So by modifying one qubit, we can change the other qubit too.
To have the complete information, we only need one more qubit to have two qubits.

Example run:
cargo run --bin superdense_coding

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use quantum_simulation::simulation::QuantumSimulation;

const QUBIT_COUNT: usize = 2;

// Sends and receives 2 bits of information using superdense coding.
fn send_and_receive(first_bit: bool, second_bit: bool) -> (bool, bool) {
    let mut simulation = QuantumSimulation::new(QUBIT_COUNT, 0u64);
    // Preparation - entangle the qubits into a Bell state (1/sqrt(2))*(|00⟩ + |11⟩).
    simulation.init_ground_state();
    simulation.hadamard(0);
    simulation.cnot(0, 1);

    // Encode the message using the first qubit only:
    // 00 -> (1/sqrt(2))*(|00⟩ + |11⟩)
    // 01 -> (1/sqrt(2))*(|00⟩ - |11⟩)
    // 10 -> (1/sqrt(2))*(|01⟩ + |10⟩)
    // 11 -> (1/sqrt(2))*(|01⟩ - |10⟩)
    if first_bit {
        simulation.pauli_z(0);
    }
    if second_bit {
        simulation.pauli_x(0);
    }

    // Send the first qubit.

    // Decode the message from the two qubits.
    simulation.cnot(0, 1);
    simulation.hadamard(0);
    let measured_states = simulation.measure_all();
    (measured_states[0], measured_states[1])
}

// Denotes the bits by a string.
fn denote(first_bit: bool, second_bit: bool) -> String {
    let mut s = String::from("");
    if second_bit {
        s = s + "1"
    } else {
        s = s + "0"
    }
    if first_bit {
        s = s + "1"
    } else {
        s = s + "0"
    }

    s
}

fn main() {
    println!("Superdense coding:");
    for second_bit in [false, true] {
        for first_bit in [false, true] {
            print!("Sending {}...", denote(first_bit, second_bit));
            let (first_received_bit, second_received_bit) = send_and_receive(first_bit, second_bit);
            assert_eq!(first_bit, first_received_bit);
            assert_eq!(second_bit, second_received_bit);
            println!(" received {}.", denote(first_received_bit, second_received_bit));
        }
    }
}
