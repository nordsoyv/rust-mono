use crate::task::Task;
use crate::util::read_file;

pub struct Task01A {}

pub struct Task01B {}

impl Task for Task01A {
  fn run(&self) {
    let file = read_file("./res/2020/task01.txt");
    let nums = file
      .lines()
      .map(|l| l.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    dbg!(&nums);
    let res = find_sum_two_numbers(2020, nums);
    let product = res.0 * res.1;
    println!("Sum is : {}", product);
  }
}

impl Task for Task01B {
  fn run(&self) {
    let file = read_file("./res/2020/task01.txt");
    let nums = file
      .lines()
      .map(|l| l.parse::<i32>().unwrap())
      .collect::<Vec<i32>>();
    dbg!(&nums);
    let res = find_sum_three_numbers(2020, nums);
    let product = res.0 * res.1 * res.2;
    println!("Sum is : {}", product);
  }
}

fn find_sum_two_numbers(target: i32, numbers: Vec<i32>) -> (i32, i32) {
  for num1 in &numbers {
    for num2 in &numbers {
      if num1 == num2 {
        continue;
      }
      if num1 + num2 == target {
        return (*num1, *num2);
      }
    }
  }
  return (0, 0);
}

fn find_sum_three_numbers(target: i32, numbers: Vec<i32>) -> (i32, i32, i32) {
  for num1 in &numbers {
    for num2 in &numbers {
      for num3 in &numbers {
        if num1 == num2 || num1 == num3 || num2 == num3 {
          continue;
        }
        if num1 + num2 + num3 == target {
          return (*num1, *num2, *num3);
        }
      }
    }
  }
  return (0, 0, 0);
}

#[test]
fn test1() {
  let nums = vec![1721, 979, 366, 299, 675, 1456];
  let result = find_sum_two_numbers(2020, nums);
  dbg!(result);
  let product = result.0 * result.1;
  assert_eq!(product, 514579);
}

#[test]
fn test2() {
  let nums = vec![1721, 979, 366, 299, 675, 1456];
  let result = find_sum_three_numbers(2020, nums);
  dbg!(result);
  let product = result.0 * result.1 * result.2;
  assert_eq!(product, 241861950);
}