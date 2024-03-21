use lexer::LexedStr;
use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstFunctionNode {
  pub name: LexedStr,
  pub children: Vec<NodeRef>,
}
