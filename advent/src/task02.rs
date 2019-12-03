use std::fs;

use crate::task::Task;

pub struct Task02A {}

pub struct Task02B {}

fn int_code_reader(filename: &str) -> Vec<i32> {
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  contents.split(",").collect::<Vec<&str>>().iter().map(|n| {
    n.parse::<i32>().unwrap()
  }).collect::<Vec<i32>>()
}

fn _print_int_code(code: &Vec<i32>) {
  for chunk in code.chunks(4) {
    println!("{:02?}", chunk);
  }
}

fn run_code(code: &mut Vec<i32>) {
  let mut instruction_pointer = 0;
  loop {
    let op = code[instruction_pointer];
    if op == 99 {
      return;
    }
    let arg1 = code[instruction_pointer + 1] as usize;
    let arg1_value = code[arg1];
    let arg2 = code[instruction_pointer + 2] as usize;
    let arg2_value = code[arg2];
    let loc = code[instruction_pointer + 3] as usize;

    match op {
      1 => {
        code[loc] = arg1_value + arg2_value
      }
      2 => {
        code[loc] = arg1_value * arg2_value
      }
      _ => panic!(format!("Unknown op code. pos: {}", instruction_pointer))
    }

    instruction_pointer += 4;
  }
}

impl Task for Task02A {
  fn run(&self) {
    let mut int_code = int_code_reader("./res/task02.txt");

    int_code[1] = 12;
    int_code[2] = 2;

    run_code(&mut int_code);
    println!("Answer in slot 0 is: {}", int_code[0]);
  }
}

impl Task for Task02B {
  fn run(&self) {
    let org_code = int_code_reader("./res/task02.txt");

    for noun in 0..99 {
      for verb in 0..99 {
        let mut int_code = org_code.clone();
        int_code[1] = noun;
        int_code[2] = verb;
        run_code(&mut int_code);
        let answer = int_code[0];
        if answer == 19690720 {
          println!("Noun {} Verb {} ", noun, verb);
          println!("Final answer: {}", (100 * noun) + verb);
          return;
        }
      }
    }
  }
}