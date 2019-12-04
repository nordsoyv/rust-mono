use std::fs;

use crate::task::Task;

pub struct Task01A {}

fn calc_fuel(weight: f32) -> f32 {
    let a1 = weight / 3f32;
    let a2 = a1.floor() - 2f32;
    return a2;
}

fn calc_full_module_weight(weight: f32) -> f32 {
    let module_fuel = calc_fuel(weight);
    let mut total_fuel = module_fuel;
    let mut rest_fuel = module_fuel;
    loop {
        let fuel_fuel = calc_fuel(rest_fuel);
        if fuel_fuel < 0f32 {
            break;
        }
        total_fuel += fuel_fuel;
        rest_fuel = fuel_fuel;
    }
    return total_fuel;
}

impl Task for Task01A {
    fn run(&self) {
        let contents =
            fs::read_to_string("./res/task01a.txt").expect("Something went wrong reading the file");

        let sum = contents
            .lines()
            .map(|l| l.parse::<f32>().unwrap())
            .map(calc_fuel)
            .fold(0f32, |acc, n| acc + n);

        println!("Fuel required is : {}", sum);
    }
}

pub struct Task01B {}

impl Task for Task01B {
    fn run(&self) {
        let contents =
            fs::read_to_string("./res/task01a.txt").expect("Something went wrong reading the file");

        let sum = contents
            .lines()
            .map(|l| l.parse::<f32>().unwrap())
            .map(calc_full_module_weight)
            .fold(0f32, |acc, n| acc + n);

        println!("Fuel required is : {}", sum);
    }
}
