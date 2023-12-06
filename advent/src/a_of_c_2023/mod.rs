mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;

use self::task01::{Task01A, Task01B};
use self::task02::{Task02A, Task02B};
use self::task03::{Task03A, Task03B};
use self::task04::{Task04A, Task04B};
use self::task05::{Task05A, Task05B};
use self::task06::{Task06A, Task06B};
use crate::task::Task;

pub fn create_2023_task(id: &str) -> Box<dyn Task> {
  match id {
    "01a" => Box::new(Task01A {}),
    "01b" => Box::new(Task01B {}),
    "02a" => Box::new(Task02A {}),
    "02b" => Box::new(Task02B {}),
    "03a" => Box::new(Task03A {}),
    "03b" => Box::new(Task03B {}),
    "04a" => Box::new(Task04A {}),
    "04b" => Box::new(Task04B {}),
    "05a" => Box::new(Task05A {}),
    "05b" => Box::new(Task05B {}),
    "06a" => Box::new(Task06A {}),
    "06b" => Box::new(Task06B {}),
    _ => panic!("No task found"),
  }
}
