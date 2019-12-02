use crate::task01::{Task01A, Task01B};
use crate::task::Task;

mod task;
mod task01;


fn main() {
  println!("Hello, world!");
  let task = std::env::args().nth(1).expect("no task given");
  println!("Task given is: {}", task);
  match task.as_str() {
    "01a" => {
      let task01a = Task01A {};
      task01a.run();
    }
    "01b" => {
      let task01b = Task01B {};
      task01b.run();
    }
    _ => println!("No task found")
  }
}
