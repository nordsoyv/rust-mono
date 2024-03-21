use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstIdentifierNode {
  pub identifier: LexedStr,
}
