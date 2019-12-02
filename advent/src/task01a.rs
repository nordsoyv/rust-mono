use std::env;
use std::fs;

use crate::task::Task;

pub struct Task01A {}

impl Task for Task01A {
  fn run(&self) {
    let contents = fs::read_to_string("./res/task01a.txt")
      .expect("Something went wrong reading the file");

    let mut lines = contents.lines();

    let sum = lines.map(|l| {
      l.parse::<f32>().unwrap()
    }).map(|n| {
      let a1 = n / 3f32;
      let a2 = a1.floor() - 2f32;
      return a2;
    }).fold(0f32, |acc, n| {
      acc + n
    });

    println!("{}", sum);
  }
}