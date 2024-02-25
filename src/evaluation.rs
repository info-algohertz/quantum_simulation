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

pub fn evaluate(measurements: Vec<Vec<bool>>) {
    let measurement_count = measurements.len();
    println!("Measurement count: {:?}", measurement_count);
    let qubit_count = measurements[0].len();
    dbg!(qubit_count);

    let mut measurement_count: HashMap<Vec<bool>, usize> = HashMap::new();

    for measurement in measurements {
        if let Some(count) = measurement_count.get(&measurement) {
            measurement_count.insert(measurement, count + 1);
        } else {
            measurement_count.insert(measurement, 1);
        }
    }

    let mut measurement_count_pairs: Vec<_> = measurement_count.into_iter().collect();
    measurement_count_pairs.sort_by(|a, b| b.1.cmp(&a.1));

    for (measurement, count) in measurement_count_pairs {
        println!("{}: {:?}", measurement_string(measurement), count);
    }

}
