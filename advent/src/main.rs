use crate::task01::{Task01A, Task01B};
use crate::task02::{Task02A, Task02B};
use crate::task::Task;

mod task;
mod task01;
mod task02;


fn main() {
  println!("Hello, world!");
  let task = std::env::args().nth(1).expect("no task given");
  println!("Task given is: {}", task);
  match task.as_str() {
    "01a" => {
      Task01A {}.run();
    }
    "01b" => {
      Task01B {}.run();
    }
    "02a" => {
      Task02A {}.run();
    }
    "02b" => {
      Task02B {}.run();
    }
    _ => println!("No task found")
  }
}
