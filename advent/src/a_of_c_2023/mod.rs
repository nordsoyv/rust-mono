use crate::{empty_task::{TaskEmptyB, TaskEmptyA}, task::Task};

pub fn create_2023_task(id: &str) -> Box<dyn Task> {
    match id {
      "01a" => {
        Box::new(TaskEmptyA {})
      }
      "01b" => {
        Box::new(TaskEmptyB {})
      }
      _ => panic!("No task found"),
    }
  }
  
