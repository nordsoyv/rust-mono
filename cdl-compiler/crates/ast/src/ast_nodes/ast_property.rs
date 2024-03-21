use crate::NodeRef;
use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstPropertyNode {
  pub name: LexedStr,
  pub child: Vec<NodeRef>,
}
