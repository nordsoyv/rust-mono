use crate::{task::Task, util::read_file};

pub struct Task09A {}
pub struct Task09B {}

impl Task for Task09A {
  fn run(&self) {
    let input = read_file("./res/2023/task09.txt");
    let mut task = Task9::parse(input.to_string());
    let res = task.solve_a();
    println!("The result is {}", res);
  }
}

impl Task for Task09B {
  fn run(&self) {
    let input = read_file("./res/2023/task09.txt");
    let mut task = Task9::parse(input.to_string());
    let res = task.solve_b();
    println!("The result is {}", res);
  }
}

#[derive(Debug)]
struct Task9 {
  pub sequences: Vec<Sequence>,
}

impl Task9 {
  pub fn parse(input: String) -> Task9 {
    let seq: Vec<Sequence> = input
      .lines()
      .map(|line| {
        line
          .split(" ")
          .map(|p| p.parse::<isize>().unwrap())
          .collect::<Vec<isize>>()
      })
      .map(|numbers| Sequence {
        numbers,
        diff_sequence: None,
      })
      .collect();
    Task9 { sequences: seq }
  }
  pub fn solve_a(&mut self) -> isize {
    for seq in &mut self.sequences {
      let child = seq.generate_diff_sequence();
      seq.diff_sequence = child;
    }
    return self
      .sequences
      .iter()
      .map(|seq| seq.get_next_number())
      .fold(0, |acc, num| acc + num);
  }

  pub fn solve_b(&mut self) -> isize {
    for seq in &mut self.sequences {
      let child = seq.generate_diff_sequence();
      seq.diff_sequence = child;
    }
    return self
      .sequences
      .iter()
      .map(|seq| seq.get_prev_number())
      .fold(0, |acc, num| acc + num);
  }
}

#[derive(Debug)]
pub struct Sequence {
  numbers: Vec<isize>,
  diff_sequence: Option<Box<Sequence>>,
}

impl Sequence {
  fn is_all_zeros(&self) -> bool {
    if self.numbers.iter().all(|n| *n == 0) {
      return true;
    }
    return false;
  }

  pub fn generate_diff_sequence(&self) -> Option<Box<Sequence>> {
    let numbers = self
      .numbers
      .windows(2)
      .map(|a| a[1] - a[0])
      .collect::<Vec<isize>>();

    let mut seq = Sequence {
      numbers,
      diff_sequence: None,
    };
    if seq.is_all_zeros() {
      return Some(Box::new(seq));
    } else {
      seq.diff_sequence = seq.generate_diff_sequence();
      return Some(Box::new(seq));
    }
  }

  pub fn get_next_number(&self) -> isize {
    if let Some(diff) = &self.diff_sequence {
      return self.numbers.last().unwrap() + diff.get_next_number();
    } else {
      return 0;
    }
  }

  fn get_prev_number(&self) -> isize {
    if let Some(diff) = &self.diff_sequence {
      return self.numbers.first().unwrap() - diff.get_prev_number();
    } else {
      return 0;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

  #[test]
  fn task_a_example() {
    let mut task = Task9::parse(TEST_INPUT.to_string());
    let res = task.solve_a();
    assert_eq!(114, res);
  }

  #[test]
  fn task_b_example() {
    let mut task = Task9::parse(TEST_INPUT.to_string());
    let res = task.solve_b();
    assert_eq!(2, res);
  }
}
