use crate::task::Task;
use crate::task01::{Task01A, Task01B};
use crate::task02::{Task02A, Task02B};
use crate::task03::{Task03A, Task03B};
use crate::task04::{Task04A, Task04B};
use crate::task05::{Task05A, Task05B};

mod int_code;
mod empty_task;
mod task;
mod task01;
mod task02;
mod task03;
mod task04;
mod task05;
mod util;

fn get_task(id: &str) -> Box<dyn Task> {
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

fn main() {
  let task_id = std::env::args().nth(1).expect("no task given");
  println!("Task given is: {}", task_id);
  let task = get_task(&task_id);
  let start_run = std::time::Instant::now();
  task.run();
  let end_run = start_run.elapsed();
  println!("Time taken: {} ms",(end_run.as_nanos() as f64) / (1000.0 * 1000.0))   ;

}
