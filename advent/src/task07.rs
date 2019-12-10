use crate::int_code::{int_code_reader, IntCode, IntCodeMachine};
use crate::task::Task;

pub struct Task07A {}

pub struct Task07B {}

//fn find_params_feedback(int_code: &IntCode) -> (i32, Vec<i32>) {
//  let mut best = 0;
//  let mut best_params = vec![];
//
//  for first_iter in 5..10 {
//    for second_iter in 5..10 {
//      if second_iter == first_iter {
//        continue;
//      }
//      for third_iter in 5..10 {
//        if third_iter == first_iter || third_iter == second_iter {
//          continue;
//        }
//        for fourth_iter in 5..10 {
//          if fourth_iter == first_iter || fourth_iter == second_iter || fourth_iter == third_iter {
//            continue;
//          }
//          for fifth_iter in 5..10 {
//            if fifth_iter == first_iter || fifth_iter == second_iter || fifth_iter == third_iter || fifth_iter == fourth_iter {
//              continue;
//            }
//
//            let mut machine1 = IntCodeMachine::new();
//            machine1.set_code(int_code.clone());
//            let mut machine2 = IntCodeMachine::new();
//            machine2.set_code(int_code.clone());
//            let mut machine3 = IntCodeMachine::new();
//            machine3.set_code(int_code.clone());
//            let mut machine4 = IntCodeMachine::new();
//            machine4.set_code(int_code.clone());
//            let mut machine5 = IntCodeMachine::new();
//            machine5.set_code(int_code.clone());
//
//            let mut input1 = vec![first_iter, 0];
//            let mut input2 = vec![second_iter];
//            let mut input3 = vec![third_iter];
//            let mut input4 = vec![fourth_iter];
//            let mut input5 = vec![first_iter];
//
//            let mut output1 = machine1.run_until_output(&mut input1);
//            input2.push(output1[0]);
//            let mut output2 = machine2.run_until_output(&mut input2);
//            input3.push(output2[0]);
//            let mut output3 = machine3.run_until_output(&mut input3);
//            input4.push(output3[0]);
//            let mut output4 = machine4.run_until_output(&mut input4);
//            input5.push(output4[0]);
//            let mut output5 = machine5.run_until_output(&mut input5);
//            //input1.push(output5[0]);
//
//            let mut last5 = 0;
//            let mut i = 0;
//            loop {
//              i += 1;
//              dbg!(i);
//              output1 = machine1.run_until_output(&mut vec![output5[0]]);
//              if output1.len()==0{
//                break;
//              }
//              dbg!(&output1);
//              output2 = machine2.run_until_output(&mut vec![output1[0]]);
//              if output2.len()==0{
//                break;
//              }
//              dbg!(&output2);
//              output3 = machine3.run_until_output(&mut vec![output2[0]]);
//              if output3.len()==0{
//                break;
//              }
//              dbg!(&output3);
//              output4 = machine4.run_until_output(&mut vec![output3[0]]);
//              if output4.len()==0{
//                break;
//              }
//              dbg!(&output4);
//              output5 = machine5.run_until_output(&mut vec![output4[0]]);
//              if output5.len() == 0 {
//                break;
//              }
//              dbg!(&output5);
//
//              last5 = output5[0];
////              dbg!(last1,last2,last3,last4,last5);
//            }
//
//            best = last5;
//            best_params = vec![first_iter, second_iter, third_iter, fourth_iter, fifth_iter];
//          }
//        }
//      }
//    }
//  }
//  (best, best_params)
//}
//

fn find_params(int_code: &IntCode) -> (i32, Vec<i32>) {
  let mut best = 0;
  let mut best_params = vec![];
  for first_iter in 0..5 {
    let mut machine1 = IntCodeMachine::new(int_code.clone());
    machine1.run(&mut vec![first_iter, 0]);

    for second_iter in 0..5 {
      if second_iter == first_iter {
        continue;
      }
      let mut machine2 = IntCodeMachine::new(int_code.clone());
      machine2.run(&mut vec![second_iter, *machine1.output.last().unwrap()]);

      for third_iter in 0..5 {
        if third_iter == first_iter || third_iter == second_iter {
          continue;
        }
        let mut machine3 = IntCodeMachine::new(int_code.clone());
        machine3.run(&mut vec![third_iter, *machine2.output.last().unwrap()]);

        for fourth_iter in 0..5 {
          if fourth_iter == first_iter || fourth_iter == second_iter || fourth_iter == third_iter {
            continue;
          }

          let mut machine4 = IntCodeMachine::new(int_code.clone());
          machine4.run(&mut vec![fourth_iter, *machine3.output.last().unwrap()]);


          for fifth_iter in 0..5 {
            if fifth_iter == first_iter || fifth_iter == second_iter || fifth_iter == third_iter || fifth_iter == fourth_iter {
              continue;
            }

            let mut machine5 = IntCodeMachine::new(int_code.clone());
            let mut input = vec![fifth_iter, *machine4.output.last().unwrap()];
            machine5.run(&mut input);

            let result = *machine5.output.last().unwrap();
            if result > best {
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
    let code = int_code_reader("./res/task07.txt");
    let (best, best_params) = find_params(&code);
    println!("Best : {}, params : {:?}", best, best_params);
  }
}

impl Task for Task07B {
  fn run(&self) {
    let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string();
    let code = input
      .split(",")
      .collect::<Vec<&str>>()
      .iter()
      .map(|n| n.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();

//    let (best, best_params) = find_params_feedback(&code);
//
//    dbg!(best, & best_params);
  }
}

#[test]
fn test_a1() {
  let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string();
  let code = input
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let (best, best_params) = find_params(&code);

  dbg!(best, & best_params);

  assert_eq!(best, 43210);
}

#[test]
fn test_a2() {
  let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string();
  let code = input
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let (best, best_params) = find_params(&code);

  dbg!(best, & best_params);

  assert_eq!(best, 54321);
  assert_eq!(best_params, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_a3() {
  let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string();
  let code = input
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let (best, best_params) = find_params(&code);

  dbg!(best, & best_params);

  assert_eq!(best, 65210);
  assert_eq!(best_params, vec![1, 0, 4, 3, 2]);
}

//
//#[test]
//fn test_b1() {
//  let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string();
//  let code = input
//    .split(",")
//    .collect::<Vec<&str>>()
//    .iter()
//    .map(|n| n.parse::<i32>().unwrap())
//    .collect::<Vec<i32>>();
//
//  let (best, best_params) = find_params_feedback(&code);
//
//  dbg!(best, & best_params);
//  assert_eq!(best, 139629729);
//  assert_eq!(best_params, vec![9, 8, 7, 6, 5]);
//}
