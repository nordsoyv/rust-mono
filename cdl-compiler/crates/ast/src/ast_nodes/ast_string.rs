use lexer::LexedStr;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum QuoteKind {
  SingleQuote,
  DoubleQuote,
}

#[derive(Debug, Serialize, Clone)]
pub struct AstStringNode {
  pub text: LexedStr,
  pub quote_kind: QuoteKind,
}
