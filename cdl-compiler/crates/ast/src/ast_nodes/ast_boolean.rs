use std::cell::Cell;

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstBooleanNode {
  value: Cell<bool>,
}

impl AstBooleanNode {
  pub fn new(value: bool) -> Self {
    Self {
      value: Cell::new(value),
    }
  }
  pub fn set(&self, value: bool) {
    self.value.set(value)
  }
  pub fn get(&self) -> bool {
    self.value.get()
  }
}
