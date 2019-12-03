use crate::task::Task;

pub struct TaskEmptyA {}
pub struct TaskEmptyB {}

impl Task for TaskEmptyA {
  fn run(&self) {
    unimplemented!()
  }
}

impl Task for TaskEmptyB {
  fn run(&self) {
    unimplemented!()
  }
}
