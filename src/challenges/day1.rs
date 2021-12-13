use colored::*;
use std::fs::File;
use std::io::*;

/// Read the raw measurements from the specified file
fn get_measurements(filename: String) -> Vec<i32> {
    // Step 1: open and unwrap a file.
    //let file = File::open("data/example_day_1").unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut result: Vec<i32> = Vec::new();

    // Step 2: loop over lines and print them.
    for line in reader.lines() {
        result.push(line.unwrap().parse::<i32>().unwrap());
    }
    result
}

/// DAY 1 Calculations
/// Caclulate the number of increases
fn calculate_increases(measurements: &Vec<i32>) -> usize {
    let increases = measurements
        .iter()
        .enumerate()
        .filter(|(index, value)| index > &0 && value > &&measurements[index - 1])
        .count();

    return increases;
}

/// DAY 2 calculations
/// Increase using a sliding algoritm
fn calculate_sliding_increase(measurements: &Vec<i32>) -> usize {
    let mut sliding_sum: Vec<i32> = Vec::new();
    for i in 0..measurements.len() - 2 {
        sliding_sum.push(measurements[i] + measurements[i + 1] + measurements[i + 2]);
    }

    return calculate_increases(&sliding_sum);
}

pub fn run() {
    let measurements = get_measurements("data/day1".to_string());
    let increases = calculate_increases(&measurements);
    println!("{}: {}", "Increased: ".blue(), increases);
    let sliding_increased = calculate_sliding_increase(&measurements);
    println!("{}: {}", "Increased (sliding): ".blue(), sliding_increased)
}
