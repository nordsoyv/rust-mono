mod task;
mod task01a;


use crate::task01a::Task01A;
use crate::task::Task;



fn main() {
    println!("Hello, world!");
    let task = std::env::args().nth(1).expect("no task given");
    println!("Task given is: {}", task);
    let mut task01a = Task01A {    };
    task01a.run();
}
