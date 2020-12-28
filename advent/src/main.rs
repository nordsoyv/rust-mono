
mod a_of_c_2019;
mod a_of_c_2020;

mod empty_task;
mod task;
mod util;

use crate::a_of_c_2019::create_2019_task;
use crate::a_of_c_2020::create_2020_task;

fn main() {
  let year = std::env::args().nth(1).expect("no year given");
  let task_id = std::env::args().nth(2).expect("no task given");
  println!("Task given is: {} , {}", task_id, year);
  let task = match year.as_str() {
    "2019" => create_2019_task(&task_id),
    "2020" => create_2020_task(&task_id),
    _ => panic!("Unknown year given")
  };
  println!("Running task {} - {}", year,task_id);
  let start_run = std::time::Instant::now();
  task.run();
  let end_run = start_run.elapsed();
  println!("Time taken: {} ms", (end_run.as_nanos() as f64) / (1000.0 * 1000.0));
}
