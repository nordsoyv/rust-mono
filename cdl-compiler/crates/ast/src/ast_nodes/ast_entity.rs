use lexer::LexedStr;
use serde::Serialize;

use crate::NodeRef;

#[derive(Debug, Serialize, Clone)]
pub struct AstEntityNode {
  pub children: Vec<NodeRef>,
  pub terms: Vec<LexedStr>,
  pub label: Option<LexedStr>,
  pub refs: Vec<LexedStr>,
  pub ident: Option<LexedStr>,
  pub entity_number: Option<f64>,
}
