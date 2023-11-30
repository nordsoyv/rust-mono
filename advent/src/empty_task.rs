use crate::task::Task;

pub struct TaskEmptyA {}
pub struct TaskEmptyB {}

impl Task for TaskEmptyA {
    fn run(&self) {
        println!("Emtpy task, not impemented yet");
    }
}

impl Task for TaskEmptyB {
    fn run(&self) {
        println!("Emtpy task, not impemented yet");
    }
}
