use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Serialize, Clone)]
pub enum QuoteKind {
  SingleQuote,
  DoubleQuote,
}

#[derive(Debug, Serialize, Clone)]
pub struct AstStringNode {
  pub text: Rc<str>,
  pub quote_kind: QuoteKind,
}


