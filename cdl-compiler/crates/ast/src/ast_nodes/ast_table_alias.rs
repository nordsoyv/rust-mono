use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AstTableAliasNode {
  pub table: LexedStr,
  pub alias: LexedStr,
}
