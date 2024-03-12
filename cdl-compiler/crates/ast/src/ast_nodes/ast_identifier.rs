use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Serialize, Clone)]
pub struct AstIdentifierNode {
  pub identifier: Rc<str>,
}
