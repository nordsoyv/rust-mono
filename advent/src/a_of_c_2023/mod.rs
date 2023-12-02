mod task01;
mod task02;

use self::task01::{Task01A, Task01B};
use self::task02::{Task02A, Task02B};
use crate::task::Task;

pub fn create_2023_task(id: &str) -> Box<dyn Task> {
  match id {
    "01a" => Box::new(Task01A {}),
    "01b" => Box::new(Task01B {}),
    "02a" => Box::new(Task02A {}),
    "02b" => Box::new(Task02B {}),
    _ => panic!("No task found"),
  }
}
