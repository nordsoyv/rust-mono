use crate::task::Task;
use std::collections::HashMap;
use crate::util::read_file;

pub struct Task06A {}

pub struct Task06B {}

impl Task for Task06A {
  fn run(&self) {
    let input = read_file("./res/2020/task06.txt");
    let groups = parse_input(&input);
    let total: usize = groups.iter()
      .map(|g| g.answers.len())
      .fold(0, |acc, n| acc + n);

    println!("Total sum of all answers: {}", total);
  }
}

impl Task for Task06B {
  fn run(&self) {
    let input = read_file("./res/2020/task06.txt");
    let groups = parse_input2(&input);
    let total: usize = groups.iter()
      .map(|g| g.get_correct_answers())
      .fold(0, |acc, n| acc + n);

    println!("Total sum of all answers for all members: {}", total);
  }
}

struct Group {
  answers: HashMap<char, bool>
}

struct Group2 {
  answers: HashMap<char, usize>,
  member_count: usize,
}

impl Group {
  pub fn new() -> Group {
    Group {
      answers: HashMap::new(),
    }
  }
  pub fn add_answer(&mut self, answer: char) {
    self.answers.insert(answer, true);
  }
}

impl Group2 {
  pub fn new() -> Group2 {
    Group2 {
      answers: HashMap::new(),
      member_count: 0,
    }
  }
  pub fn add_answer(&mut self, answer: char) {
    let current = {
      self.answers.get(&answer)
    };

    match current {
      Some(count) => self.answers.insert(answer, count + 1),
      None => self.answers.insert(answer, 1)
    };
  }

  pub fn add_member(&mut self) {
    self.member_count += 1;
  }

  pub fn get_correct_answers(&self) -> usize {
    let mut total = 0;
    for answer in &self.answers {
      if *answer.1 == self.member_count {
        total += 1;
      }
    }
    total
  }
}


fn parse_input(input: &str) -> Vec<Group> {
  let mut current = Group::new();
  let mut result = vec![];
  for line in input.lines() {
    if line == "" {
      result.push(current);
      current = Group::new();
      continue;
    }
    line.chars().for_each(|c| current.add_answer(c));
  }
  result.push(current);
  return result;
}

fn parse_input2(input: &str) -> Vec<Group2> {
  let mut current = Group2::new();
  let mut result = vec![];
  for line in input.lines() {
    if line == "" {
      result.push(current);
      current = Group2::new();
      continue;
    }
    current.add_member();
    line.chars().for_each(|c| current.add_answer(c));
  }
  result.push(current);
  return result;
}

#[cfg(test)]
mod test {
  use crate::a_of_c_2020::task06::{parse_input, parse_input2};

  #[test]
  fn test_set1() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let groups = parse_input(input);
    let total: usize = groups.iter()
      .map(|g| g.answers.len())
      .fold(0, |acc, n| acc + n);

    assert_eq!(total, 11);
  }

  #[test]
  fn test_set2() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    let groups = parse_input2(input);
    let total: usize = groups.iter()
      .map(|g| g.get_correct_answers())
      .fold(0, |acc, n| acc + n);

    assert_eq!(total, 6);
  }
}