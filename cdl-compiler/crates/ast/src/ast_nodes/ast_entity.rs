use std::cell::RefCell;

use lexer::LexedStr;
use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstEntityNode {
  pub children: RefCell<Vec<NodeRef>>,
  pub terms: Vec<LexedStr>,
  pub label: Option<LexedStr>,
  pub refs: Vec<LexedStr>,
  pub ident: Option<LexedStr>,
  pub entity_number: Option<f64>,
}

impl AstEntityNode {
  pub fn add_child(&self, child: NodeRef) {
    self.children.borrow_mut().push(child)
  }
  pub fn get_number_of_children(&self) -> usize {
    self.children.borrow().len()
  }
}
