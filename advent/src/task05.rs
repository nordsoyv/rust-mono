use std::io;

use crate::int_code::{int_code_reader, IntCodeMachine};
use crate::task::Task;

pub struct Task05A {}

pub struct Task05B {}

impl Task for Task05A {
  fn run(&self) {
    let int_code = int_code_reader("./res/task05.txt");
    let mut machine = IntCodeMachine::new();
    machine.set_code(int_code);
    let input = b"1\n";
    let mut output = Vec::new();
    machine.run(&input[..], &mut output);
    println!("{}", String::from_utf8(output).unwrap());
  }
}

impl Task for Task05B {
  fn run(&self) {
    let int_code = int_code_reader("./res/task05.txt");
    let mut machine = IntCodeMachine::new();
    machine.set_code(int_code);
    let input = b"5\n";
    let mut output = Vec::new();
    machine.run(&input[..], &mut output);
    println!("{}", String::from_utf8(output).unwrap());
  }
}
