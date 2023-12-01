use self::task01::{Task01A, Task01B};

mod task01;
use crate::task::Task;

pub fn create_2023_task(id: &str) -> Box<dyn Task> {
  match id {
    "01a" => Box::new(Task01A {}),
    "01b" => Box::new(Task01B {}),
    _ => panic!("No task found"),
  }
}
