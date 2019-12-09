use crate::task::Task;
use crate::int_code::IntCodeMachine;

pub struct Task07A {}

pub struct Task07B {}

impl Task for Task07A {
  fn run(&self) {

  }
}

impl Task for Task07B {
  fn run(&self) {
    unimplemented!()
  }
}

#[test]
fn test_01() {
  let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string();
  let code = input
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let mut machine1 = IntCodeMachine::new();
  machine1.set_code(code.clone());
  let output1 = machine1.run(&mut vec![4, 0]);


  let mut machine2 = IntCodeMachine::new();
  machine2.set_code(code.clone());
  let output2 = machine2.run(&mut vec![3, output1[0]]);

  let mut machine3 = IntCodeMachine::new();
  machine3.set_code(code.clone());
  let output3 = machine3.run(&mut vec![2, output2[0]]);

  let mut machine4 = IntCodeMachine::new();
  machine4.set_code(code.clone());
  let output4 = machine4.run(&mut vec![1, output3[0]]);


  let mut machine5 = IntCodeMachine::new();
  machine5.set_code(code.clone());
  let output5 = machine5.run(&mut vec![0, output4[0]]);
  assert_eq!(output5[0], 43210);
