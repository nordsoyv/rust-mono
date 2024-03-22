use std::cell::RefCell;

use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstScriptNode {
  pub children: RefCell<Vec<NodeRef>>,
}

impl AstScriptNode {
  pub fn add_child(&self, child: NodeRef) {
    self.children.borrow_mut().push(child)
  }
}