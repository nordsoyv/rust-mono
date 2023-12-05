use crate::task::Task;

pub struct TaskEmptyA {}
pub struct TaskEmptyB {}

impl Task for TaskEmptyA {
    fn run(&self) {
        println!("Empty task, not implemented yet");
    }
}

impl Task for TaskEmptyB {
    fn run(&self) {
        println!("Empty task, not implemented yet");
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_example() {
  }
}
