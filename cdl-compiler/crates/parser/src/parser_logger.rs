use std::cell::RefCell;

use log::trace;

pub trait ParserLogger: std::fmt::Debug {
  fn start_group(&self, text: &str);
  fn trace(&self, text: &str);
  fn end_group(&self, text: &str);
}

#[derive(Debug)]
struct LogGroup {
  #[allow(dead_code)]
  name: String,
}

#[derive(Debug)]
pub struct TraceLogger {
  log_groups: RefCell<Vec<LogGroup>>,
}

impl TraceLogger {
  fn get_indent(&self) -> String {
    (0..self.log_groups.borrow().len())
      .map(|_| " |")
      .collect::<String>()
  }
  #[allow(dead_code)]
  pub fn new() -> TraceLogger {
    TraceLogger {
      log_groups: RefCell::new(Vec::new()),
    }
  }
}

impl ParserLogger for TraceLogger {
  fn start_group(&self, text: &str) {
    trace!("{} {}", self.get_indent(), text);
    self.log_groups.borrow_mut().push(LogGroup {
      name: text.to_owned(),
    });
  }
  fn trace(&self, text: &str) {
    trace!("{} {}", self.get_indent(), text);
  }
  fn end_group(&self, text: &str) {
    let _group = self.log_groups.borrow_mut().pop().unwrap();
    trace!("{} {}", self.get_indent(), text);
  }
}

#[derive(Debug)]
pub struct MockLogger {}

impl MockLogger {
  #[allow(dead_code)]
  pub fn new() -> MockLogger {
    MockLogger {}
  }
}

impl ParserLogger for MockLogger {
  fn start_group(&self, _text: &str) {}
  fn trace(&self, _text: &str) {}
  fn end_group(&self, _text: &str) {}
}
