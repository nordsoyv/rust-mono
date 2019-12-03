use std::fs;

use crate::task::Task;
use crate::util;

pub struct Task03A {}
pub struct Task03B {}

impl Task for Task03A {
  fn run(&self) {
    let content = util::read_file("./res/task03.txt");
    let lines = content.lines();
    for l in lines {
      println!("{}", l);
    }
  }
}

impl Task for Task03B {
  fn run(&self) {
    unimplemented!()
  }
}
