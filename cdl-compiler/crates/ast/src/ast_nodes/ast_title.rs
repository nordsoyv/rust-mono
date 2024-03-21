use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstTitleNode {
  pub title: LexedStr,
}
