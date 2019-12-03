use std::fs;
use std::str::Lines;

pub fn read_file(filename: &str) -> String {
  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
  return contents;
}
