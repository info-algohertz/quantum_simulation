/*
Module to evaluate and display the results of the quantum measurements.

Copyright © 2024 AlgoHertz. All rights reserved.
*/

use std::collections::HashMap;

fn measurement_string(measurement: Vec<bool>) -> String {
    let mut result = String::from("|");
    for value in measurement.into_iter().rev() {
        if value {
            result.push('1');
        } else {
            result.push('0');
        }
    }
    result.push('>');
    result
}

fn measurement_wildcard(qubit_count: usize, qubit_number: usize) -> String {
    let mut result = String::from("|");
    for i in (0..qubit_count).rev() {
        if qubit_number == i {
            result.push('1');
        } else {
            result.push('*');
        }
    }
    result.push('>');
    result
}

pub fn evaluate(measurements: Vec<Vec<bool>>) {
    println!("Quantum simulation results");
    let qubit_count = measurements[0].len();
    println!("Qubit count: {:?}", qubit_count);
    let measurement_count = measurements.len();
    println!("Measurement count: {:?}", measurement_count);

    let mut measurement_count_map: HashMap<Vec<bool>, usize> = HashMap::new();
    let mut one_counts = vec![0usize; qubit_count];

    for measurement in measurements {
        for i in 0..qubit_count {
            if measurement[i] {
                one_counts[i] += 1;
            }
        }

        if let Some(count) = measurement_count_map.get(&measurement) {
            measurement_count_map.insert(measurement, count + 1);
        } else {
            measurement_count_map.insert(measurement, 1);
        }
    }

    let mut measurement_count_pairs: Vec<_> = measurement_count_map.into_iter().collect();
    measurement_count_pairs.sort_by(|a, b| b.1.cmp(&a.1));

    for (measurement, count) in measurement_count_pairs {
        let probability_pct: f64 = 100.0 * count as f64 / measurement_count as f64;
        println!("{}: {:?}%", measurement_string(measurement), probability_pct);
    }

    for qubit_number in 0..qubit_count {
        let probability_pct: f64 = 100.0 * one_counts[qubit_number] as f64 / measurement_count as f64;
        println!("{:?}. {}: {:?}%", qubit_number, measurement_wildcard(qubit_count, qubit_number), probability_pct);
    }
}
