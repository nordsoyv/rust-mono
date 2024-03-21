use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstVPathNode {
  pub table: Option<LexedStr>,
  pub variable: Option<LexedStr>,
  pub function: Option<LexedStr>,
  pub is_hierarchy: bool,
}
