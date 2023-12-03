use crate::task::Task;

pub struct TaskEmptyA {}
pub struct TaskEmptyB {}

impl Task for TaskEmptyA {
    fn run(&self) {
        println!("Emtpy task, not impemented yet");
    }
}

impl Task for TaskEmptyB {
    fn run(&self) {
        println!("Emtpy task, not impemented yet");
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_example() {
  }
}
