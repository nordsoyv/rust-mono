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
  pub left: NodeRef,
  pub right: NodeRef,
}
