use lexer::LexedStr;
use serde::Serialize;
use std::cell::RefCell;

#[derive(Debug, Clone, Serialize)]
pub struct AstColorNode {
  color: RefCell<LexedStr>,
}

impl AstColorNode {
  pub fn new(color: LexedStr) -> Self {
    Self {
      color: RefCell::new(color),
    }
  }

  pub fn get_color(&self) -> LexedStr {
    self.color.borrow().clone()
  }
  pub fn set_color(&self, color: LexedStr) {
    self.color.replace(color);
  }
}
