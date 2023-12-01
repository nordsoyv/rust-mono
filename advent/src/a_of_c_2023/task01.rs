use crate::{task::Task, util::read_file};

pub struct Task01A {}
pub struct Task01B {}

impl Task for Task01A {
  fn run(&self) {
    let content = read_file("./res/2023/task01a.txt");
    let result = count_numbers(content);
    println!("The result is {}", result);
  }
}

impl Task for Task01B {
  fn run(&self) {
    let content = read_file("./res/2023/task01a.txt");
    let result = count_numbers(content);
    println!("The result is {}", result);
  }
}

fn count_numbers(input: String) -> u32 {
  input
    .lines()
    .map(find_first_and_last)
    .map(|numbers| numbers.0 * 10 + numbers.1)
    .fold(0, |sum, number| sum + number)
}

fn find_first_and_last(line: &str) -> (u32, u32) {
  let mut first = 0;
  let mut last = 0;
  for i in 0..=line.len() {
    if let Some(digit) = find_digit(&line[0..i]) {
      first = digit;
      break;
    }
  }

  for i in (0..=line.len()).rev() {
    if let Some(digit) = find_digit(&line[i..line.len()]) {
      last = digit;
      break;
    }
  }
  (first, last)
}

fn find_digit(buffer: &str) -> Option<u32> {
  if buffer.contains("one") {
    return Some(1);
  }
  if buffer.contains("two") {
    return Some(2);
  }
  if buffer.contains("three") {
    return Some(3);
  }
  if buffer.contains("four") {
    return Some(4);
  }
  if buffer.contains("five") {
    return Some(5);
  }
  if buffer.contains("six") {
    return Some(6);
  }
  if buffer.contains("seven") {
    return Some(7);
  }
  if buffer.contains("eight") {
    return Some(8);
  }
  if buffer.contains("nine") {
    return Some(9);
  }
  for c in buffer.chars() {
    if c.is_ascii_digit() {
      return Some(c.to_digit(10).unwrap());
    }
  }
  return None;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_example() {
    let input = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
      .to_string();
    let result = count_numbers(input);
    assert_eq!(142, result);
  }
  #[test]
  fn task_b_example() {
    let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
      .to_string();
    let result = count_numbers(input);
    assert_eq!(281, result);
  }

  #[test]
  fn test_find_first_and_last() {
    assert_eq!((2, 9), find_first_and_last("two1nine"));
    assert_eq!((8, 3), find_first_and_last("eightwothree"));
    assert_eq!((1, 3), find_first_and_last("abcone2threexyz"));
    assert_eq!((2, 4), find_first_and_last("xtwone3four"));
    assert_eq!((4, 2), find_first_and_last("4nineeightseven2"));
    assert_eq!((1, 4), find_first_and_last("zoneight234"));
    assert_eq!((7, 6), find_first_and_last("7pqrstsixteen"));
    assert_eq!((4, 4), find_first_and_last("bvcz4"));
  }
}
