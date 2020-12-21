use crate::task::Task;
use crate::util::read_file;

pub struct Task02A {}

pub struct Task02B {}

impl Task for Task02A {
  fn run(&self) {
    let content = read_file("./res/2020/task02.txt");
    let result = content
      .lines()
      .map(|l| parse_line(l))
      .map(|(policy, password)| check_password(policy, password))
      .map(|valid| match valid {
        true => 1,
        _ => 0
      }).fold(0, |acc, n| acc + n);
    println!("Password matching policy: {}", result);
  }
}

impl Task for Task02B {
  fn run(&self) {
    let content = read_file("./res/2020/task02.txt");
    let result = content
      .lines()
      .map(|l| parse_line_alt(l))
      .map(|(policy, password)| check_password_alt(policy, password))
      .map(|valid| match valid {
        true => 1,
        _ => 0
      }).fold(0, |acc, n| acc + n);
    println!("Password matching alternate policy: {}", result);
  }
}

#[derive(Debug, PartialEq)]
struct Policy {
  min: i32,
  max: i32,
  letter: char,
}

#[derive(Debug, PartialEq)]
struct PolicyAlt {
  pos1: usize,
  pos2: usize,
  letter: char,
}

fn parse_policy(min_max: &str, letter: &str) -> Policy {
  let m = min_max.split('-').map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  Policy { min: m[0], max: m[1], letter: letter.chars().collect::<Vec<char>>()[0] }
}

fn parse_policy_alt(pos1_pos2: &str, letter: &str) -> PolicyAlt {
  let m = pos1_pos2.split('-').map(|l| l.parse::<usize>().unwrap()).collect::<Vec<usize>>();
  PolicyAlt { pos1: m[0] - 1, pos2: m[1] - 1, letter: letter.chars().collect::<Vec<char>>()[0] }
}

fn parse_line(line: &str) -> (Policy, &str) {
  let parts = line.split(" ").collect::<Vec<&str>>();
  let policy = parse_policy(parts[0], parts[1]);
  return (policy, parts[2]);
}

fn parse_line_alt(line: &str) -> (PolicyAlt, &str) {
  let parts = line.split(" ").collect::<Vec<&str>>();
  let policy = parse_policy_alt(parts[0], parts[1]);
  return (policy, parts[2]);
}

fn check_password(policy: Policy, password: &str) -> bool {
  let num_matches = password.matches(policy.letter).count();
  if num_matches < policy.min as usize {
    return false;
  }
  if num_matches > policy.max as usize {
    return false;
  }
  true
}

fn check_password_alt(policy: PolicyAlt, password: &str) -> bool {
  let mut found = 0;
  if password.chars().nth(policy.pos1).unwrap() == policy.letter {
    found += 1
  }
  if password.chars().nth(policy.pos2).unwrap()  == policy.letter {
    found += 1;
  }
  if found == 1 {
    return true
  }
  false
}

#[test]
fn parse_policy_test() {
  let p = parse_policy("1-3", "a:");
  assert_eq!(p, Policy { min: 1, max: 3, letter: 'a' });
}

#[test]
fn check_password_test() {
  assert_eq!(check_password(Policy { min: 1, max: 3, letter: 'a' }, "abcde"), true);
  assert_eq!(check_password(Policy { min: 1, max: 3, letter: 'b' }, "cdefg"), false);
  assert_eq!(check_password(Policy { min: 2, max: 9, letter: 'c' }, "ccccccccc"), true);
}

#[test]
fn parse_policy_alt_test() {
  let p = parse_policy_alt("1-3", "a:");
  assert_eq!(p, PolicyAlt { pos1: 0, pos2: 2, letter: 'a' });
}

#[test]
fn parse_line_alt_test() {
  let p = parse_line("1-3 a: aaaaaasdf");
  dbg!(p);
}

#[test]
fn check_password_alt_test() {
  assert_eq!(check_password_alt(PolicyAlt { pos1: 0, pos2: 2, letter: 'a' }, "abcde"), true);
  assert_eq!(check_password_alt(PolicyAlt { pos1: 0, pos2: 2, letter: 'b' }, "cdefg"), false);
  assert_eq!(check_password_alt(PolicyAlt { pos1: 1, pos2: 8, letter: 'c' }, "ccccccccc"), false);
}

/*

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
 */