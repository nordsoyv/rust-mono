use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstFormulaNode {
  pub children: Vec<NodeRef>,
}
