use std::cell::RefCell;

use crate::NodeRef;
use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstPropertyNode {
  pub name: LexedStr,
  pub children: RefCell<Vec<NodeRef>>,
}
impl AstPropertyNode {
  pub(crate) fn add_property(&self, child: NodeRef) {
    self.children.borrow_mut().push(child);
  }
  pub fn new(name: LexedStr) -> Self {
    Self {
      name,
      children: RefCell::new(vec![]),
    }
  }
}
