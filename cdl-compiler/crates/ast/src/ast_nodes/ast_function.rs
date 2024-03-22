use std::cell::RefCell;

use lexer::LexedStr;
use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstFunctionNode {
  pub name: LexedStr,
  pub children: RefCell<Vec<NodeRef>>,
}
impl AstFunctionNode {
    pub(crate) fn add_argument(&self, child: NodeRef) {
      self.children.borrow_mut().push(child)
    }
}
