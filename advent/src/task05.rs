
use crate::int_code::{int_code_reader, IntCodeMachine};
use crate::task::Task;

pub struct Task05A {}

pub struct Task05B {}

impl Task for Task05A {
  fn run(&self) {
    let int_code = int_code_reader("./res/task05.txt");
    let mut machine = IntCodeMachine::new();
    machine.set_code(int_code);
    let output = machine.run(&mut vec![1]);
    println!("{}", output[0]);
  }
}

impl Task for Task05B {
  fn run(&self) {
    let int_code = int_code_reader("./res/task05.txt");
    let mut machine = IntCodeMachine::new();
    machine.set_code(int_code);
    let output =machine.run(&mut vec![5]);
    println!("{}", output[0]);
  }
}
