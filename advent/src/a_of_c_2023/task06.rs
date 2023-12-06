use crate::task::Task;

pub struct Task06A {}
pub struct Task06B {}

impl Task for Task06A {
  fn run(&self) {
    let input = r"Time:        55     82     64     90
  Distance:   246   1441   1012   1111"
      .to_string();
    let res = self.solve(input);
    println!("The result is {}", res);
  }
}

impl Task for Task06B {
  fn run(&self) {
    let input = r"Time:        55     82     64     90
  Distance:   246   1441   1012   1111"
      .to_string();
    let res = self.solve(input);
    println!("The result is {}", res);
  }
}

impl Task06A {
  pub fn solve(&self, input: String) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i64> = lines[0]
      .replace("Time:", "")
      .split(" ")
      .filter(|p| !p.is_empty())
      .map(|p| p.trim().parse::<i64>().unwrap())
      .collect();
    let dist: Vec<i64> = lines[1]
      .replace("Distance:", "")
      .split(" ")
      .filter(|p| !p.is_empty())
      .map(|p| p.trim().parse::<i64>().unwrap())
      .collect();
    let mut num_wins_arr = vec![];
    for index in 0..dist.len() {
      let current_record = dist[index];
      let mut num_wins = 0;
      for charge_time in 0..times[index] {
        let next_dist = race_boat(charge_time, times[index]);
        if next_dist > current_record {
          num_wins += 1;
        }
      }
      num_wins_arr.push(num_wins);
    }
    return num_wins_arr.iter().fold(1, |sum, elem| sum * elem);
  }
}
impl Task06B {
  pub fn solve(&self, input: String) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let race_time: i64 = lines[0]
      .replace("Time:", "")
      .replace(" ", "")
      .parse::<i64>()
      .unwrap();
    let race_record: i64 = lines[1]
      .replace("Distance:", "")
      .replace(" ", "")
      .parse::<i64>()
      .unwrap();

    let mut num_wins = 0;
    for charge_time in 0..race_time {
      let next_dist = race_boat(charge_time, race_time);
      if next_dist > race_record {
        num_wins += 1;
      }
    }
    return num_wins;
  }
}

fn race_boat(charge_time: i64, race_time: i64) -> i64 {
  let speed = charge_time;
  let race_remaing_time = race_time - charge_time;
  if race_remaing_time <= 0 {
    return 0;
  }
  return race_remaing_time * speed;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_example() {
    let input = r"Time:      7  15   30
Distance:  9  40  200"
      .to_string();
    let task = Task06A {};
    let res = task.solve(input);
    assert_eq!(288, res);
  }

  #[test]
  fn task_b_example() {
    let input = r"Time:      7  15   30
Distance:  9  40  200"
      .to_string();
    let task = Task06B {};
    let res = task.solve(input);
    assert_eq!(71503, res);
  }
}
