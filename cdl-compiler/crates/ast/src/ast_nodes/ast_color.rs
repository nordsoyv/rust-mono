use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AstColorNode {
  pub color: LexedStr,
}

impl AstColorNode {
  pub fn new(color: LexedStr) -> Self {
    Self {
      color,
    }
  }
}
