use crate::task::Task;
use crate::a_of_c_2020::task01::{Task01A, Task01B};
use crate::a_of_c_2020::task02::{Task02B, Task02A};
use crate::a_of_c_2020::task03::{Task03A, Task03B};
use crate::a_of_c_2020::task04::{Task04A, Task04B};
use crate::a_of_c_2020::task05::{Task05A, Task05B};
use crate::a_of_c_2020::task06::{Task06A, Task06B};
use crate::a_of_c_2020::task07::{Task07A, Task07B};
use crate::a_of_c_2020::task08::{Task08A, Task08B};

mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod task06;
mod task07;
mod task08;

pub fn create_2020_task(id: &str) -> Box<dyn Task> {
  match id {
    "01a" => {
      Box::new(Task01A {})
    }
    "01b" => {
      Box::new(Task01B {})
    }
    "02a" => {
      Box::new(Task02A {})
    }
    "02b" => {
      Box::new(Task02B {})
    }
    "03a" => {
      Box::new(Task03A {})
    }
    "03b" => {
      Box::new(Task03B {})
    }
    "04a" => {
      Box::new(Task04A {})
    }
    "04b" => {
      Box::new(Task04B {})
    }
    "05a" => {
      Box::new(Task05A {})
    }
    "05b" => {
      Box::new(Task05B {})
    }
    "06a" => {
      Box::new(Task06A {})
    }
    "06b" => {
      Box::new(Task06B {})
    }
    "07a" => {
      Box::new(Task07A {})
    }
    "07b" => {
      Box::new(Task07B {})
    }
    "08a" => {
      Box::new(Task08A {})
    }
    "08b" => {
      Box::new(Task08B {})
    }
    _ => panic!("No task found"),
  }
}
