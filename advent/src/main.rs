mod a_of_c_2019;
mod a_of_c_2020;
mod empty_task;
mod task;
mod util;

use crate::a_of_c_2019::create_2019_task;
use crate::a_of_c_2020::create_2020_task;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  #[arg(short, long, value_name = "YEAR")]
  year: String,

  #[arg(short, long, value_name = "TASK")]
  task: String,
}

fn main() {
  let cli = Cli::parse();
  let year: String = cli.year;
  let task_id = cli.task;
  println!("Task given is: {} :: {}", &year, &task_id);
  let task = match year.as_str() {
    "2019" => create_2019_task(&task_id),
    "2020" => create_2020_task(&task_id),
    _ => panic!("Unknown year given"),
  };
   println!("Running task {} :: {}", &year,&task_id);
   let timer = std::time::Instant::now();
   task.run();
   let end_run = timer.elapsed();
   println!("Time taken: {} ms", (end_run.as_nanos() as f64) / (1000.0 * 1000.0));
}
