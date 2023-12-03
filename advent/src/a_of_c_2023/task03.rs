use std::ops::Range;

use crate::{task::Task, util::read_file};

pub struct Task03A {}
pub struct Task03B {}

impl Task for Task03A {
  fn run(&self) {
    let input = read_file("./res/2023/task03a.txt");
    let lines = EngineSchematics::from_string(input);    
    let r = lines.find_numbers();
    dbg!(&r[0..10]);
    let sum : i32= r.into_iter().sum();
    println!("The result is {}", sum);
  }
}

impl Task for Task03B {
  fn run(&self) {
    println!("Emtpy task, not impemented yet");
  }
}

#[derive(Debug)]
struct EngineSchematics {
  lines: Vec<String>,
}

impl EngineSchematics {
  fn from_string(l: String) -> EngineSchematics {
    EngineSchematics {
      lines: l.lines().map(|s| s.to_string()).collect(),
    }
  }
  fn get_char_at(&self, line_num: isize, pos: isize) -> char {
    if line_num >= 0 && line_num < (self.lines.len() as isize) {
      let selected_line = &self.lines[line_num as usize];
      if pos >= 0 && pos < selected_line.len() as isize {
        return selected_line.chars().nth(pos as usize).unwrap();
      }
    }
    return '.';
  }

  fn is_symbol_at(&self, line_num: isize, pos: isize) -> bool {
    let c = self.get_char_at(line_num, pos);
    if c.is_digit(10) {
      return false;
    }
    if c == '.' {
      return false;
    }
    return true;
  }

  fn find_numbers(&self) -> Vec<i32> {
    let mut numbers = vec![];
    for line_num in 0..self.lines.len() {
      let num_ranges = self.find_digit_spans_in_line(line_num as isize);
      for num_range in num_ranges {
        let mut is_valid_number = false;
        for pos in num_range.clone() {
          if self.is_symbol_at((line_num as isize) - 1, (pos as isize) -1  ) {
            is_valid_number = true;
            break;
          }
          if self.is_symbol_at((line_num as isize) - 1, pos as isize) {
            is_valid_number = true;
            break;
          }
          if self.is_symbol_at((line_num  as isize)- 1, (pos + 1) as isize) {
            is_valid_number = true;
            break;
          }
          if self.is_symbol_at(line_num as isize, (pos + 1) as isize) {
            is_valid_number = true;
            break;
          }
          if self.is_symbol_at(line_num as isize, (pos  as isize )- 1 ) {
            is_valid_number = true;
            break;
          }
          if self.is_symbol_at((line_num + 1) as isize, (pos as isize) - 1) {
            is_valid_number = true;
            break;
          }
          if self.is_symbol_at((line_num + 1) as isize, pos as isize) {
            is_valid_number = true;
            break;
          }
          if self.is_symbol_at((line_num + 1) as isize, (pos + 1) as isize) {
            is_valid_number = true;
            break;
          }
        }
        if is_valid_number {
            let line = self.lines[line_num].as_str();
            let r = line[num_range].parse::<i32>().unwrap();
            numbers.push(r);
        }
      }
    }
    numbers
  }

  fn find_digit_spans_in_line(&self, line_num: isize) -> Vec<Range<usize>> {
    let mut ranges = vec![];
    let mut start_range = 0;
    let mut is_in_number = false;
    for (pos, c) in self.lines[line_num as usize].char_indices() {
      if is_in_number {
        if c.is_digit(10) {
          continue;
        }
        ranges.push(start_range..pos);
        is_in_number = false;
      } else {
        if c.is_digit(10) {
          is_in_number = true;
          start_range = pos;
        }
      }
    }
    if is_in_number {
        ranges.push(start_range..self.lines[line_num as usize].len());
    }
    ranges
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_lines() {
    let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
      .to_string();
    let lines = EngineSchematics::from_string(input);
    assert_eq!('4', lines.get_char_at(0, 0));
    assert_eq!('.', lines.get_char_at(-10, -1));
    assert_eq!('*', lines.get_char_at(1, 3));
    assert!(lines.is_symbol_at(1, 3));
    assert!(!lines.is_symbol_at(-1, 3));
    assert!(!lines.is_symbol_at(0, 0));
    let r = lines.find_numbers();
    let sum : i32= r.into_iter().sum();
    assert_eq!(4361, sum)
  }

  #[test]
  fn read_lines2() {
    let input = r"
12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"
      .to_string();
    let lines = EngineSchematics::from_string(input);
    let r = lines.find_numbers();
    let sum : i32= r.into_iter().sum();
    assert_eq!(925, sum)
  }

}
