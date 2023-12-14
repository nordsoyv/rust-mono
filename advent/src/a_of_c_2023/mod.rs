mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod task07;
mod task08;
mod task09;
mod task11;

use self::task01::{Task01A, Task01B};
use self::task02::{Task02A, Task02B};
use self::task03::{Task03A, Task03B};
use self::task04::{Task04A, Task04B};
use self::task05::{Task05A, Task05B};
use self::task06::{Task06A, Task06B};
use self::task07::{Task07A, Task07B};
use self::task08::{Task08A, Task08B};
use self::task09::{Task09A, Task09B};
use self::task11::{Task11A, Task11B};
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
    "07a" => Box::new(Task07A {}),
    "07b" => Box::new(Task07B {}),
    "08a" => Box::new(Task08A {}),
    "08b" => Box::new(Task08B {}),
    "09a" => Box::new(Task09A {}),
    "09b" => Box::new(Task09B {}),
    "11a" => Box::new(Task09A {}),
    "11b" => Box::new(Task09B {}),
    _ => panic!("No task found"),
  }
}
