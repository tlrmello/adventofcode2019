use math::round;
use std::fs;

pub fn get_orig_fuel() -> Vec<f64> {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let masses: Vec<f64> = input
        .split("\n")
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    let mut fuel_count: Vec<f64> = Vec::new();
    for x in &masses {
        let y = round::floor(x / 3.0, 0) - 2.0;
        fuel_count.push(y);
    }
    fuel_count
}

pub fn get_additive_fuel(start_fuel: &Vec<f64>) -> Vec<f64> {
    let mut inner_fuel_count: Vec<f64> = Vec::new();
    let mut outer_fuel_count: Vec<f64> = Vec::new();
    for x in start_fuel {
        let mut y = *x;
        while y > 0.0 {
            y = round::floor(y / 3.0, 0) - 2.0;
            if y > 0.0 {
                inner_fuel_count.push(y);
            }
        }
        let sum_inner: f64 = inner_fuel_count.iter().sum();
        outer_fuel_count.push(sum_inner);
        inner_fuel_count.clear();
    }
    outer_fuel_count
}

fn main() {
    let start_fuel = get_orig_fuel();
    let added_fuel = get_additive_fuel(&start_fuel);
    let total_fuel_start: f64 = start_fuel.iter().sum();
    let total_fuel_additive: f64 = added_fuel.iter().sum();
    let total_fuel: f64 = total_fuel_start + total_fuel_additive;
    println!("{:#?}", total_fuel);
}
