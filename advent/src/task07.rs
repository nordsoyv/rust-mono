use crate::task::Task;
use crate::int_code::IntCodeMachine;

pub struct Task07A {}

pub struct Task07B {}

impl Task for Task07A {
  fn run(&self) {
    unimplemented!()
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

  println!("{:?}", code);

    let mut machine1 = IntCodeMachine::new();
    let input1 = b"4\n0\n";
    let mut output1 = Vec::new();
    machine1.set_code(code.clone());
    machine1.run( &input1[..], &mut output1);

    let res1_string = String::from_utf8(output1[..output1.len()-2)]).unwrap();

//    let res1_num = res1_string.parse::<i32>().unwrap();

    println!("{}", res1_string);



//    let mut machine2 = IntCodeMachine::new();
//    let input2 = b"3\n" + output1[0] + "\n";
//    let mut output2 = Vec::new();
//    machine2.set_code(code.clone());
//    machine2.run( &input2[..], &mut output2);
//
//    println!("{}", String::from_utf8(output2).unwrap());



}