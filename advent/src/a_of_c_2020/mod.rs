use crate::task::Task;
use crate::a_of_c_2020::task01::{Task01A, Task01B};
use crate::a_of_c_2020::task02::{Task02B, Task02A};
use crate::a_of_c_2020::task03::{Task03A, Task03B};
use crate::a_of_c_2020::task04::{Task04A, Task04B};
use crate::a_of_c_2020::task05::{Task05A, Task05B};

mod task01;
mod task02;
mod task03;
mod task04;
mod task05;

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
    _ => panic!("No task found"),
  }
}
