use crate::task::Task;
use crate::int_code::{IntCodeMachine, int_code_reader, IntCode};

pub struct Task07A {}

pub struct Task07B {}

fn find_params(int_code : &IntCode) -> (i32,Vec<i32>) {
  let mut best = 0;
  let mut best_params = vec![];
  for first_iter in 0..10 {
    let mut machine1 = IntCodeMachine::new();
    machine1.set_code(int_code.clone());
    let output1 = machine1.run(&mut vec![first_iter, 0]);

    for second_iter in 0..10 {
      if second_iter == first_iter {
        continue;
      }
      let mut machine2 = IntCodeMachine::new();
      machine2.set_code(int_code.clone());
      let output2 = machine2.run(&mut vec![second_iter, output1[0]]);

      for third_iter in 0..10 {
        if third_iter == first_iter || third_iter == second_iter {
          continue;
        }
        let mut machine3 = IntCodeMachine::new();
        machine3.set_code(int_code.clone());
        let output3 = machine3.run(&mut vec![third_iter, output2[0]]);

        for fourth_iter in 0..10 {
          if fourth_iter == first_iter || fourth_iter == second_iter || fourth_iter == third_iter {
            continue;
          }

          let mut machine4 = IntCodeMachine::new();
          machine4.set_code(int_code.clone());
          let output4 = machine4.run(&mut vec![fourth_iter, output3[0]]);


          for fifth_iter in 0..10 {
            if fifth_iter == first_iter || fifth_iter == second_iter || fifth_iter == third_iter || fifth_iter == fourth_iter {
              continue;
            }

            let mut machine5 = IntCodeMachine::new();
            machine5.set_code(int_code.clone());
//            dbg!(first_iter, second_iter, third_iter, fourth_iter, fifth_iter, output4[0]);
            let mut input = vec![fifth_iter, output4[0]];
//            dbg!(&input);
            let output5 = machine5.run(&mut input);

            let result = output5[0];
//            dbg!(result);
            if result > best {
//              dbg!(best, result);
              best = result;
              best_params = vec![first_iter, second_iter, third_iter, fourth_iter, fifth_iter];
            }
          }
        }
      }
    }
  }

  (best, best_params)
}

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

  let (best, best_params) = find_params(&code);

  dbg!(best, &best_params);

  assert_eq!(best, 43210);
}

#[test]
fn test_02() {
  let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string();
  let code = input
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let (best, best_params) = find_params(&code);

  dbg!(best, &best_params);

  assert_eq!(best, 54321);
  assert_eq!(best_params, vec![0,1,2,3,4]);
}

#[test]
fn test_03() {
  let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string();
  let code = input
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let (best, best_params) = find_params(&code);

  dbg!(best, &best_params);

  assert_eq!(best, 65210);
  assert_eq!(best_params, vec![1,0,4,3,2]);
}

