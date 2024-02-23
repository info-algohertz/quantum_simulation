use std::collections::HashMap;

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

    for (measurement, count) in measurement_count {
        println!("{:?}: {:?}", measurement, count);
    }

}
