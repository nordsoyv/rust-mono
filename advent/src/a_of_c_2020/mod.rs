use crate::task::Task;
use crate::a_of_c_2020::task01::{Task01A, Task01B};

mod task01;

pub fn create_2020_task(id: &str) -> Box<dyn Task> {
    match id {
        "01a" => {
            Box::new(Task01A {})
        }
        "01b" => {
            Box::new(Task01B {})
        }
        _ => panic!("No task found"),
    }
}
