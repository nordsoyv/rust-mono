use std::collections::HashMap;
use num::integer::lcm;
use crate::{task::Task, util::read_file};

pub struct Task08A {}
pub struct Task08B {}

impl Task for Task08A {
  fn run(&self) {
    let input = read_file("./res/2023/task08.txt");
    let task = TaskA::parse(input.to_string());
    let res = task.solve_a();
    println!("The result is {}", res);
  }
}

impl Task for Task08B {
  fn run(&self) {
    let input = read_file("./res/2023/task08.txt");
    let task = TaskA::parse(input.to_string());
    let res = task.solve_b();
    println!("The result is {}", res);
  }
}

#[derive(Debug)]
struct TaskA {
  path: Vec<Dir>,
  map: HashMap<String, (String, String, String)>,
}

impl TaskA {
  fn parse(input: String) -> TaskA {
    let lines: Vec<&str> = input.lines().collect();

    let path: Vec<Dir> = lines[0]
      .chars()
      .map(|c| match c {
        'R' => Dir::Right,
        'L' => Dir::Left,
        _ => panic!("unknown dir  {}", c),
      })
      .collect();
    let mut map: HashMap<String, (String, String, String)> = HashMap::new();
    for line in &lines[2..] {
      let name = line[0..3].to_string();
      let left = line[7..10].to_string();
      let rigth = line[12..15].to_string();
      map.insert(name.clone(), (name, left, rigth));
    }
    TaskA { path, map }
  }

  fn solve_a(&self) -> usize {
    let mut length = 0;
    let mut node = self
      .map
      .get("AAA")
      .expect("Could not find node with name AAA");
    loop {
      if node.0 == "ZZZ" {
        return length;
      }
      let dir = &self.path[(length) % self.path.len()];
      match dir {
        Dir::Left => node = self.map.get(&node.1).unwrap(),
        Dir::Right => node = self.map.get(&node.2).unwrap(),
      }
      length += 1;
    }
  }

  fn solve_b(&self) -> usize {
    let nodes: Vec<&(String, String, String)> = self
      .map
      .keys()
      .filter(|key| key.ends_with("A"))
      .map(|key| self.map.get(key).unwrap())
      .collect();

      let mut path_lengths: Vec<usize> = vec![];

    for node in &nodes {
      let mut length = 0;
      let mut current_node = node.clone();
      loop {
        if current_node.0.ends_with("Z") {
          path_lengths.push(length);
          break;
        }
        let dir = &self.path[(length) % self.path.len()];
        current_node = match dir {
          Dir::Left => self.map.get(&current_node.1).unwrap(),
          Dir::Right => self.map.get(&current_node.2).unwrap(),
        };
        length += 1;
      }
    }

    let  mut lcm_curr: usize = 1;
    for length in path_lengths {
        lcm_curr = lcm(lcm_curr,length);
    }
    lcm_curr
  }
}

#[derive(Debug)]
enum Dir {
  Left,
  Right,
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

  const TEST_INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

  #[test]
  fn task_a_example() {
    let task = TaskA::parse(TEST_INPUT.to_string());
    let num = task.solve_a();
    assert_eq!(6, num);
  }
  #[test]
  fn task_b_example() {
    let task = TaskA::parse(TEST_INPUT2.to_string());
    let num = task.solve_b();
    assert_eq!(6, num);
  }
}
