use crate::task::Task;

pub struct Task04A {}

pub struct Task04B {}

fn check_valid_number(num: i32) -> bool {
  let s = num.to_string();
  let digits: Vec<_> = s.chars().map(|d| d.to_digit(10).unwrap()).collect();

  for i in 0..digits.len() - 1 {
    if digits[i + 1] < digits[i] {
      return false;
    }
  }

  for i in 0..digits.len() - 1 {
    if digits[i] == digits[i + 1] {
      return true;
    }
  }
  return false;
}

fn check_valid_number_ex(num: i32) -> bool {
  let s = num.to_string();
  let digits: Vec<_> = s.chars().map(|d| d.to_digit(10).unwrap()).collect();

  for i in 0..digits.len() - 1 {
    if digits[i + 1] < digits[i] {
      return false;
    }
  }

  let mut runs = vec![];
  let mut current_run = vec![];
  current_run.push(digits[0]);
  for i in 1..(digits.len()) {
    let digit = digits[i];
    if digit == current_run[0] {
      current_run.push(digit);
    } else {
      runs.push(current_run);
      current_run = vec![];
      current_run.push(digit);
    }
  }
  runs.push(current_run);


  for run in &runs {
    if run.len() == 2 {
      return true;
    }
  }

  return false;
}


impl Task for Task04A {
  fn run(&self) {
    let mut total_valid = 0;
    for num in 278384..824795 {
      let valid = check_valid_number(num);
      if valid {
        total_valid += 1;
      }
    }
    println!("Total candidate passwords: {}", total_valid);
  }
}

impl Task for Task04B {
  fn run(&self) {
    let mut total_valid = 0;
    for num in 278384..824795 {
      let valid = check_valid_number_ex(num);
      if valid {
        total_valid += 1;
      }
    }
    println!("Total candidate passwords: {}", total_valid);
  }
}


#[test]
fn test_valid() {
  assert_eq!(check_valid_number(111111), true, "111111 should be valid");
}

#[test]
fn test_valid2() {
  assert_eq!(check_valid_number(112345), true, "112345 should be valid");
}

#[test]
fn test_valid3() {
  assert_eq!(check_valid_number(111122), true, "111122 should be valid");
}

#[test]
fn test_valid4() {
  assert_eq!(check_valid_number(123456), false, "123456 should not be valid");
}

#[test]
fn test_valid5() {
  assert_eq!(check_valid_number(223450), false, "223450 should not be valid");
}

#[test]
fn test_valid6() {
  assert_eq!(check_valid_number(123789), false, "123789 should not be valid");
}

#[test]
fn test_valid7() {
  assert_eq!(check_valid_number(123444), true, "123444 should be valid");
}


#[test]
fn test_valid_ex() {
  assert_eq!(check_valid_number_ex(111111), false, "111111 should not be valid");
}

#[test]
fn test_valid_ex2() {
  assert_eq!(check_valid_number_ex(112345), true, "112345 should be valid");
}

#[test]
fn test_valid_ex3() {
  assert_eq!(check_valid_number_ex(111122), true, "111122 should be valid");
}

#[test]
fn test_valid_ex4() {
  assert_eq!(check_valid_number_ex(123456), false, "123456 should not be valid");
}

#[test]
fn test_valid_ex5() {
  assert_eq!(check_valid_number_ex(223450), false, "223450 should not be valid");
}

#[test]
fn test_valid_ex6() {
  assert_eq!(check_valid_number_ex(123789), false, "123789 should not be valid");
}

#[test]
fn test_valid_ex7() {
  assert_eq!(check_valid_number_ex(123444), false, "123444 should not be valid");
}
