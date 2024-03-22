use std::cell::Cell;

use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub enum Operator {
  Plus,
  Minus,
  Mul,
  Div,
  Equal,
  And,
  Or,
  NotEqual,
  LessThan,
  LessThanOrEqual,
  MoreThan,
  MoreThanOrEqual,
}

#[derive(Debug, Serialize, Clone)]
pub struct AstOperatorNode {
  pub operator: Operator,
  pub left: Cell<NodeRef>,
  pub right: Cell<NodeRef>,
}
impl AstOperatorNode {
  pub(crate) fn add_right(&self, child: NodeRef) {
    self.right.set(child);
  }
  pub fn new(operator: Operator, left: NodeRef, right: NodeRef) -> Self {
    AstOperatorNode {
      operator,
      left: Cell::new(left),
      right: Cell::new(right),
    }
  }
}
